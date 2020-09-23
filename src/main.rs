pub mod graph;
pub mod headings;
pub mod node;
pub mod options;
pub mod tex;
pub mod topic;
pub mod yaml;
use crate::graph::{
	build_dag_from_nodes, remove_indirect_predecessors, topological_sort,
};
use crate::headings::{
	add_heading_titles_to_nodes, compute_min_dag_costs, set_heading_depth,
};
use crate::node::Node;
use crate::options::Options;
use crate::tex::{compile_pdf, write_bib, write_to_tex};
use crate::topic::{compute_ordering, create_topic, Topic};
use crate::yaml::read_from_yaml;
use std::{
	cell::RefCell, cmp::max, cmp::min, collections::HashMap,
	collections::HashSet, env, path::Path, process::Command, rc::Rc,
};
#[macro_use]
extern crate clap;
use clap::App;
use time::PreciseTime;

fn build_graph_wrapper(
	root: Rc<RefCell<Node<Topic>>>,
	nodes: &mut HashMap<String, Rc<RefCell<Node<Topic>>>>,
	options: &Options,
	sdepth: i64,
) {
	{
		let mut pbranch: HashSet<String> = HashSet::new();
		let mut sbranch: HashSet<String> = HashSet::new();
		build_dag_from_nodes(
			root.clone(),
			nodes,
			&mut pbranch,
			&mut sbranch,
			read_from_yaml,
			create_topic,
			sdepth,
		);
	}

	// Remove indirect predecessors to generate unique DAG and compute
	// costs accurately
	for n in nodes.values() {
		remove_indirect_predecessors(n.clone());
	}

	// Compute DAG costs
	root.borrow_mut().compute_dag_cost();

	// Sort branches for topological sort (default is to sort branches so
	// that generated document presents topics in an order that traverses
	// critical path first, more suitable for reference/textbook
	// generation; user may select "lowest hanging fruit" ordering, more
	// suitable for tasks)
	for n in nodes.values() {
		n.borrow_mut()
			.sort_predecessor_branches(options.reverse, compute_ordering);
	}
}

/// The main function that executes when tok is called from the command
/// line
fn main() -> std::io::Result<()> {
	// Get start time to measure duration to output to user
	let start_time = PreciseTime::now();

	// Get command line options and arguments
	let command_line_options = load_yaml!("cli.yaml");
	let matches = App::from_yaml(command_line_options).get_matches();
	let options = Options::new(matches.clone());

	// Create root node
	let root_path = String::from("//");
	let root = Node::new(&root_path.clone(), Topic::new());

	// Do not attempt to include root node in final document
	root.borrow_mut().sorted = true;

	// Create root node and register file names from command line
	let mut nodes: HashMap<String, Rc<RefCell<Node<Topic>>>> =
		HashMap::new();
	nodes.insert(root.borrow().path.clone(), root.clone());
	for filename in options.files.iter() {
		let clean_filename = filename
			.replace("../", "")
			.replace("..\\", "")
			.replace("./", "")
			.replace(".\\", "");
		root.borrow_mut().req.push(clean_filename.to_string());
	}

	// Load nodes and construct DAG; if nodes don't have deadlines, then
	// this graph will be preserved
	println!("Building Directed Acyclic Graph (ignoring cycles)...");
	build_graph_wrapper(
		root.clone(),
		&mut nodes,
		&options,
		options.sdepth,
	);

	// Sort nodes while preserving dependency relationships; deadlines
	// override branch traversal; otherwise, cost influences order of
	// branch traversal
	let sorted_nodes = {
		// Create list of nodes with deadline, sorted by deadline
		let nodes_with_deadlines = {
			let mut values: Vec<Rc<RefCell<Node<Topic>>>> =
				nodes.values().cloned().collect();
			values.sort_by(|a, b| compute_ordering(options.reverse, a, b));
			let filtered_values: Vec<Rc<RefCell<Node<Topic>>>> = values
				.into_iter()
				.filter(|x| x.borrow().data().deadline.is_some())
				.collect();
			filtered_values
		};

		// Build DAGs from nodes with deadlines, sort nodes within each DAG
		// respecting dependency relationships, and sort lists respecting
		// deadlines
		let mut dl_list: Vec<Rc<RefCell<Node<Topic>>>> = vec![];
		if nodes_with_deadlines.len() > 0 {
			// Destroy edges in DAG; will prevent nodes with multiple
			// successors from being excluded in final document
			for n in nodes_with_deadlines.iter() {
				// Construct directed acyclic graph
				root.borrow_mut().reset();
				root.borrow_mut().req.clear();
				root.borrow_mut().req.push(n.borrow().path.clone());
				build_graph_wrapper(root.clone(), &mut nodes, &options, 0);

				// for each task with deadline, run topological sort
				let mut tmp_list = topological_sort(root.clone());
				tmp_list.append(&mut dl_list);
				dl_list = tmp_list.clone();
			}

			// Construct DAG again
			root.borrow_mut().reset();
			root.borrow_mut().req.clear();
			nodes.insert(root.borrow().path.clone(), root.clone());
			for filename in options.files.iter() {
				let clean_filename = filename
					.replace("../", "")
					.replace("..\\", "")
					.replace("./", "")
					.replace(".\\", "");
				root.borrow_mut().req.push(clean_filename.to_string());
			}
			build_graph_wrapper(
				root.clone(),
				&mut nodes,
				&options,
				options.sdepth,
			);
		}

		// run topological sort on nodes without deadlines, without adding
		// tasks with deadlines or their predecessors a second time, and
		// concatenate lists
		let mut sorted_nodes: Vec<Rc<RefCell<Node<Topic>>>> =
			topological_sort(root.clone());
		sorted_nodes.append(&mut dl_list);
		sorted_nodes
	};
	println!("Finished sorting nodes in DAG.");

	// Generate headings
	if options.generate_headings == true || options.extra_headings == true
	{
		// Rank costs
		let mut ranked_costs: Vec<usize> = sorted_nodes
			.clone()
			.into_iter()
			.map(|node| node.borrow().dag_cost())
			.collect();
		ranked_costs.sort();

		// Minimum cost for a node to qualify to have a heading of any depth
		let min_cost =
			compute_min_dag_costs(options.extra_headings, ranked_costs);
		set_heading_depth(root.clone(), &min_cost);
	}

	// Add headings, included manually added headings
	let max_heading_depth = {
		let mut mhd: usize = 0;
		if options.generate_headings == true
			|| options.extra_headings == true
		{
			add_heading_titles_to_nodes(&sorted_nodes);
			for node in sorted_nodes.clone() {
				mhd = max(mhd, node.borrow().data().heading_depth);
			}
		}
		// Even if we don't generate headings, we still need to provide this
		// argument to `tex::write_tex`
		min(mhd, 6)
	};

	// Terminal output to view organization of topics without
	// generating/viewing PDF
	println!("");
	println!("========================================");
	println!("Order of files in document:");
	println!("COST | HEADING DEPTH | FILE | LABEL");
	for n in sorted_nodes.iter().rev() {
		for heading_title in n.borrow().data().heading_titles.clone() {
			if heading_title.is_empty() == false {
				println!(" ---- {}", heading_title);
			}
		}
		println!(
			"{} | {} | {} | {}",
			n.borrow().dag_cost(),
			n.borrow().data().heading_depth,
			n.borrow().path,
			n.borrow().data().label,
		);
	}
	println!("{} total nodes", sorted_nodes.len());
	let time_to_build_dag = start_time.to(PreciseTime::now());
	println!(
		"Time to build DAG: {} ms.",
		time_to_build_dag.num_milliseconds()
	);

	// Create document source file (TeX/MD) and compile document
	// (TeX->PDF, MD->HTML)
	println!("========================================");
	// make directories for output
	Command::new("mkdir")
		.arg("../output")
		.output()
		.expect("Could not create output/ directory");

	// Symlink directories for media (e.g. code listings, images, etc.)
	if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
		// Get parent path (project root)
		let a = env::current_dir()?;
		let b = a.to_path_buf();
		let c = b.parent();
		let parent_path = c.unwrap();

		// Symlink code
		let code_path = parent_path.display().to_string() + "/code";
		let code_link = parent_path.display().to_string() + "/output/code";
		if Path::new(&code_path).exists() && !Path::new(&code_link).exists()
		{
			std::os::unix::fs::symlink(code_path, code_link)?;
		}

		// Symlink images
		let images_path = parent_path.display().to_string() + "/images";
		let images_link =
			parent_path.display().to_string() + "/output/images";
		if Path::new(&images_path).exists()
			&& !Path::new(&images_link).exists()
		{
			std::os::unix::fs::symlink(images_path, images_link)?;
		}
	}

	// FIXME: std::os::windows not detected by compiler
	// else if cfg!(target_os = "windows") {
	// 	std::os::windows::fs::symlink_dir("../code/", "../output/images/")?;
	// }

	// Write text stored in nodes to tex file
	if options.make_tex == true {
		write_to_tex(
			&options,
			&sorted_nodes,
			options.files.clone(),
			max_heading_depth,
		);
		write_bib(&sorted_nodes);

		// Report time
		println!(
			"Time to generate TEX file: {} ms.",
			(start_time.to(PreciseTime::now()) - time_to_build_dag)
				.num_milliseconds()
		);
		println!("========================================");

		// Compile PDF
		if options.make_pdf == true {
			compile_pdf(&options);
			println!(
				"Time to generate TEX+PDF: {} ms.",
				(start_time.to(PreciseTime::now()) - time_to_build_dag)
					.num_milliseconds()
			);
		}
	}
	println!("Finished.");

	// End
	Ok(())
}
