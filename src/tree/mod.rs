use std::{
	cell::RefCell, collections::HashMap, fs::File, io::prelude::*, rc::Rc,
};

use crate::node::Node;

/// Add successor node to tree;
/// root and node must not refer to the same object
pub fn add_successor<T>(
	root: Rc<RefCell<Node<T>>>,
	node: Rc<RefCell<Node<T>>>,
	successor: Rc<RefCell<Node<T>>>,
) {
	add_predecessor(root.clone(), successor.clone());
	add_predecessor(successor.clone(), node.clone());
}

/// Add predecessor node to tree
pub fn add_predecessor<T>(
	node: Rc<RefCell<Node<T>>>,
	predecessor: Rc<RefCell<Node<T>>>,
) {
	// Get number of predecessors
	let num_predecessors = node.borrow_mut().num_predecessors();

	// Add predecessor
	node.borrow_mut().push_predecessor(predecessor.clone());

	// Ensure additional predecessor is not a duplicate
	// predecessor.borrow_mut().incr_num_successors();
	if num_predecessors < node.borrow_mut().num_predecessors() {
		predecessor.borrow_mut().incr_num_successors();
		let update = predecessor.borrow().tree_cost();
		node.borrow_mut().add_to_tree_cost(update);
	}
}

/// Load file containing node data
pub fn load<T, Y>(
	map: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
	filename: &String,
	read_into_node: fn(&String, Y) -> Rc<RefCell<Node<T>>>,
	read_data: fn(&String) -> Y,
	// options: &Options,
) {
	if map.contains_key(filename) == false {
		println!("Reading {}", filename);
		let mut file = File::open(&filename).expect("Can't open file!");
		let mut contents = String::new();
		file
			.read_to_string(&mut contents)
			.expect("Cannot read data");

		// Deserialize
		let yaml_content: Y = read_data(&contents);

		// Create node and add reference to node in map
		map.insert(
			filename.to_string(),
			read_into_node(filename, yaml_content),
		);

		// TODO: Make modifications to node based on options
	}
}

/// Load nodes declared in after and before, starting with after
pub fn build_tree_root_to_leaf<T, Y>(
	current_node: Rc<RefCell<Node<T>>>,
	map: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
	pbranch: &mut HashMap<String, ()>,
	sbranch: &mut HashMap<String, ()>,
	read_into_node: fn(&String, Y) -> Rc<RefCell<Node<T>>>,
	read_data: fn(&String) -> Y,
	depth: i32,
) {
	// Iterate over predecessor paths
	let after_paths = current_node.borrow_mut().after.clone();
	for it in after_paths.iter() {
		// Do not add predecessors if they form a cycle
		if pbranch.contains_key(it) == false {
			// Add file name to list of paths on branch
			pbranch.insert(it.clone(), ());

			// Load node only if not yet loaded
			load(map, it, read_into_node, read_data);

			// Recursion
			let predecessor_node = map.get(it).unwrap().clone();
			build_tree_leaf_to_root(
				predecessor_node.clone(),
				map,
				pbranch,
				sbranch,
				read_into_node,
				read_data,
				depth,
			);
			add_predecessor(current_node.clone(), predecessor_node.clone());

			// Exit branch
			pbranch.remove(it);
		}
	}
}

/// Load nodes declared in before; current_node must not be root
pub fn build_tree_leaf_to_root<T, Y>(
	current_node: Rc<RefCell<Node<T>>>,
	map: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
	pbranch: &mut HashMap<String, ()>,
	sbranch: &mut HashMap<String, ()>,
	read_into_node: fn(&String, Y) -> Rc<RefCell<Node<T>>>,
	read_data: fn(&String) -> Y,
	mut depth: i32,
) {
	if depth != 0 {
		// Iterate over successor paths
		let before_paths = current_node.borrow_mut().before.clone();
		for it in before_paths.iter() {
			// Do not add successors if they form a cycle
			if sbranch.contains_key(it) == false {
				// Add file name to list of paths on branch
				sbranch.insert(it.clone(), ());

				// Load node only if not yet loaded
				load(map, it, read_into_node, read_data);

				// Recursion
				let successor_node = map.get(it).unwrap().clone();
				build_tree_leaf_to_root(
					successor_node.clone(),
					map,
					pbranch,
					sbranch,
					read_into_node,
					read_data,
					depth,
				);
				if depth > 0 {
					depth -= 1;
				}
				let root = map.get(&"//".to_string()).unwrap().clone();
				add_successor(
					root,
					current_node.clone(),
					successor_node.clone(),
				);

				// Exit branch
				sbranch.remove(it);
			}
		}
	}
	build_tree_root_to_leaf(
		current_node.clone(),
		map,
		pbranch,
		sbranch,
		read_into_node,
		read_data,
		depth,
	);
}

/// Modified Depth First Search to add predecessors to tree; requires
/// branches to be sorted
pub fn topological_sort<T>(
	node: Rc<RefCell<Node<T>>>
) -> Vec<Rc<RefCell<Node<T>>>> {
	let mut stack = vec![node.clone()];
	let mut sorted_nodes = vec![];
	while stack.is_empty() == false {
		let v = stack.pop().unwrap();
		if v.borrow().is_discovered() == false {
			// This condition prevents nodes from being marked discovered
			// prematurely
			if v.borrow().has_single_successor() {
				v.borrow_mut().mark_discovered();
			}

			// This is part of the normal DFS
			for w in v.borrow().predecessors().iter() {
				if w.borrow().has_multiple_successors() {
					w.borrow_mut().decr_num_successors();
				} else {
					stack.push(w.clone());
				}
			}

			if v.borrow().is_discovered() == true {
				sorted_nodes.push(v.clone());
			}
		}
	}
	sorted_nodes
}
