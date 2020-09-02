pub mod graph;
pub mod headings;
pub mod node;
pub mod options;
pub mod tex;
pub mod topic;
pub mod yaml;
use crate::graph::{
	build_dag_from_nodes, remove_indirect_predecessors, topological_sort,
	topological_sort_deadlines,
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

/// The main function that executes when tok is called from the command
/// line
fn main() -> std::io::Result<()> {
	// Measure duration to output to user
	let start_time = PreciseTime::now();

	// Get command line options and arguments
	let command_line_options = load_yaml!("cli.yaml");
	let matches = App::from_yaml(command_line_options).get_matches();
	let options = Options::new(matches.clone());

	// Create root node
	let root_path = String::from("//");
	let root = Node::new(&root_path, Topic::new());

	// Construct directed acyclic graph
	let mut nodes: HashMap<String, Rc<RefCell<Node<Topic>>>> =
		HashMap::new();
	nodes.insert(root_path, root.clone());
	for filename in options.files.clone() {
		let clean_filename = filename.replace("../", "").replace("./", "");
		root.borrow_mut().req.push(clean_filename.to_string());
	}
	println!("Building Directed Acyclic Graph (ignoring cycles)...");
	{
		let mut pbranch: HashSet<String> = HashSet::new();
		let mut sbranch: HashSet<String> = HashSet::new();
		build_dag_from_nodes(
			root.clone(),
			&mut nodes,
			&mut pbranch,
			&mut sbranch,
			read_from_yaml,
			create_topic,
		);
	}

	// Remove indirect predecessors to generate unique DAG and compute
	// costs accurately
	for (_, n) in nodes.clone() {
		remove_indirect_predecessors(n.clone());
	}

	// Compute DAG costs
	root.borrow_mut().compute_dag_cost();

	// Create list of nodes, sorted by deadline
	let nodes_by_deadline = {
		let mut values: Vec<Rc<RefCell<Node<Topic>>>> =
			nodes.values().cloned().collect();
		values.sort_by(|a, b| compute_ordering(options.reverse, a, b));
		values
	};

	// {
	// 	println!(
	// 		"DAG costs when nodes are sorted by deadline, reverse=={}",
	// 		options.reverse.to_string()
	// 	);
	// 	let costs = nodes_by_deadline
	// 		.clone()
	// 		.into_iter()
	// 		.map(|x| x.borrow().dag_cost());
	// 	let deadlines = nodes_by_deadline
	// 		.clone()
	// 		.into_iter()
	// 		.map(|x| x.borrow().data().deadline.clone());
	// 	let it: Vec<(usize, Option<Vec<usize>>)> =
	// 		costs.zip(deadlines).collect();
	// 	for i in it.iter() {
	// 		if i.1.is_none() {
	// 			println!("{}", i.0.to_string());
	// 		} else {
	// 			let date = i.1.clone().unwrap();
	// 			println!(
	// 				"{}, {}-{}-{}",
	// 				i.0.to_string(),
	// 				date[0].to_string(),
	// 				date[1].to_string(),
	// 				date[2].to_string(),
	// 			);
	// 		}
	// 	}
	// }

	// TODO: propagate deadlines from leaf to root

	// Sort branches for topological sort (default is to sort branches so
	// that generated document presents topics in an order that traverses
	// critical path first, more suitable for reference/textbook
	// generation; user may select "lowest hanging fruit" ordering, more
	// suitable for tasks)
	for (_, n) in nodes.clone() {
		n.borrow_mut()
			.sort_predecessor_branches(options.reverse, compute_ordering);
	}
	// nodes.into_iter().map(|pair| -> () {
	// 	let n = pair.1;
	// 	n.borrow_mut()
	// 		.sort_predecessor_branches(options.reverse, compute_ordering);
	// });

	// Sort nodes while preserving dependency relationships; sort by cost
	// except where deadlines are defined
	let sorted_nodes = {
		// Construct DAGs rooted at each node with deadline
		let mut sorted_nodes_with_deadlines = vec![];
		'search_dl: for n in nodes_by_deadline.clone() {
			if n.borrow().data().deadline.is_some() {
				let mut s = topological_sort_deadlines(n.clone());
				s.reverse();
				if s.is_empty() == false {
					sorted_nodes_with_deadlines.append(&mut s);
				}
			} else {
				sorted_nodes_with_deadlines.reverse();
				break 'search_dl;
			}
		}

		// Construct DAG from remaining nodes
		let mut sorted_nodes_no_deadlines =
			topological_sort_deadlines(root.clone());
		sorted_nodes_no_deadlines.append(&mut sorted_nodes_with_deadlines);
		sorted_nodes_no_deadlines
		// sorted_nodes_with_deadlines
	};

	// Topological sort
	// let sorted_nodes_no_deadlines = topological_sort(root.clone());

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
	println!("Finished.");

	// End
	Ok(())
}
