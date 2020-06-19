pub mod graph;
pub mod node;
pub mod options;
pub mod tex;
pub mod topic;
pub mod yaml;
use crate::graph::{
	build_dag_from_nodes, remove_indirect_predecessors, topological_sort,
};
use crate::node::Node;
use crate::options::Options;
use crate::tex::{compile_pdf, write_bib, write_to_tex};
use crate::topic::{create_topic, Topic};
use crate::yaml::read_from_yaml;
use std::{
	cell::RefCell, collections::HashMap, collections::HashSet,
	process::Command, rc::Rc,
};
#[macro_use]
extern crate clap;
use clap::App;
use time::PreciseTime;

/// The main function that executes when tok is called from the command line
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
		root.borrow_mut().after.push(clean_filename.to_string());
	}
	println!("Building Directed Acyclic Graph");
	println!("from files (ignoring cycles)...");
	println!("");

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
	println!("");

	// Remove indirect predecessors to compute costs accurately
	for (_, n) in nodes.clone() {
		remove_indirect_predecessors(n.clone());
	}
	// println!("Last topics should be:");
	// for p in root.borrow().predecessors() {
	// 	println!("{}", p.borrow().path);
	// }
	// for (_, node) in nodes.clone() {
	// 	// remove_indirect_predecessors(node.clone());
	// 	println!("Predecessors of {}:", node.borrow().path);
	// 	for p in node.borrow().predecessors() {
	// 		println!("  {}", p.borrow().path);
	// 	}
	// }

	// Compute DAG costs
	root.borrow_mut().compute_dag_cost();

	// Sort branches for topological sort (default is critical path; user
	// may select lowest hanging fruit)
	for (_, n) in nodes.clone() {
		n.borrow_mut().sort_predecessor_branches(options.reverse);
	}

	// Topological sort
	let sorted_nodes = topological_sort(root.clone());
	// TODO: also print labels
	println!("Order of files in document:");
	println!("");
	for n in sorted_nodes.iter().rev() {
		println!(
			// "{}, {}, {}",
			// "{}, {}",
			"{}",
			// n.borrow().dag_cost(),
			n.borrow().path,
			// n.borrow().num_successors()
		);
	}

	// Create document source file (TeX/MD) and compile document (TeX->PDF, MD->HTML)
	if options.make_pdf == true {
		println!("========================================");
		// make directories for output
		// TODO: Use symlinks instead
		let mut mkdir_cmd = Command::new("mkdir");
		let mkdir_code_args = ["../output/code/"];
		let mkdir_images_args = ["../output/images/"];
		mkdir_cmd
			.arg("../output")
			.output()
			.expect("Could not create output/ directory");
		mkdir_cmd
			.args(&mkdir_code_args)
			.output()
			.expect("Could not create output/code/ directory");
		mkdir_cmd
			.args(&mkdir_images_args)
			.output()
			.expect("Could not create output/images/ directory");

		// #[cfg(target_os = "macos")]
		// std::os::unix::fs::symlink("../code/", "../output/code/")?;
		// std::os::unix::fs::symlink("../code/", "../output/images/")?;
		// #[cfg(target_os = "windows")]
		// std::os::windows::fs::symlink_dir("../code/", "../output/code/")?;
		// std::os::windows::fs::symlink_dir("../code/", "../output/images/")?;

		// copy directories for figures, snippets, etc.
		let _cp_cls_status = Command::new("sh")
			.arg("-c")
			.arg("cp -r ../texinput/*.cls ../output/")
			.status()
			.unwrap();
		let _cp_bst_status = Command::new("sh")
			.arg("-c")
			.arg("cp -r ../texinput/*.bst ../output/")
			.status()
			.unwrap();
		let _cp_images_status = Command::new("sh")
			.arg("-c")
			.arg("cp -r ../images/* ../output/images")
			.status()
			.unwrap();
		let _cp_code_status = Command::new("sh")
			.arg("-c")
			.arg("cp -r ../code/* ../output/code")
			.status()
			.unwrap();

		// Write text stored in nodes to tex file
		write_to_tex(&options, &sorted_nodes, options.files.clone());
		write_bib(&sorted_nodes);

		// Report time
		println!("========================================");
		println!(
			"Time to generate tex file: {} ms.",
			(start_time.to(PreciseTime::now())).num_milliseconds()
		);

		// Compile PDF
		compile_pdf(&options);
	}
	println!("Finished.");

	// End
	Ok(())
}
