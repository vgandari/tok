use std::{cell::RefCell, collections::HashMap, rc::Rc};
#[macro_use]
extern crate clap;
use clap::App;
use time::PreciseTime;

pub mod node;
pub mod options;
pub mod tex;
pub mod tree;
pub mod yaml;
use node::Node;
use options::Options;
use tex::{compile_pdf, write_to_tex};
use tree::{build_tree_root_to_leaf, topological_sort};
use yaml::{read_from_yaml, update_fields, YamlData};
extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;

fn main() {
	// Measure duration to output to user
	let start_time: PreciseTime = PreciseTime::now();

	// Get command line options and arguments
	let cli_yaml = load_yaml!("cli.yaml");
	let matches = App::from_yaml(cli_yaml).get_matches();
	let options = Options::new(matches.clone());
	// Create root node
	let rootpath = "//".to_string();
	let root = Node::new(&rootpath, YamlData::new());

	// Add argument paths to root predecessor paths
	for f in options.files.clone() {
		root.borrow_mut().after.push(f.to_string());
	}

	// Remove any duplicates from arguments
	root.borrow_mut().dedup_after();
	let mut after_copy = root.borrow().after.clone();
	for it in &mut after_copy {
		if it.contains("./") {
			let _0 = it.remove(0);
			let _1 = it.remove(0);
		}
	}
	root.borrow_mut().after = after_copy;

	// Create record of nodes loaded so far and nodes loaded in current
	// branch to break cycles
	let mut map: HashMap<String, Rc<RefCell<Node<YamlData>>>> =
		HashMap::new();
	let mut pbranch: HashMap<String, ()> = HashMap::new();
	map.insert(rootpath.clone(), root.clone());
	pbranch.insert(rootpath.clone(), ());

	// Load all predecessor nodes and add to tree
	println!("=====================================");
	println!("Building tree ...");
	build_tree_root_to_leaf(
		root.clone(),
		&mut map,
		&mut pbranch,
		&mut HashMap::new(),
		update_fields,
		read_from_yaml,
		&options,
	);
	// Process tree branches
	root.borrow_mut().sort_predecessor_branches(options.reverse);
	// Sort nodes in order that their content will be printed to pdf
	let sorted_nodes = topological_sort(root.clone());
	println!("=====================================");
	println!("Result of topological sort (reversed)");
	for n in sorted_nodes.iter() {
		println!("{}", n.borrow().path);
	}
	println!("=====================================");

	// Write text stored in nodes to tex file
	write_to_tex(options, sorted_nodes);
	println!(
		"Time to generate tex file: {} ms.",
		(start_time.to(PreciseTime::now())).num_milliseconds()
	);
	println!("=====================================");
	// Compile PDF
	compile_pdf();
	println!("Finished.");
}
