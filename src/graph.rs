use crate::node::Node;
use crate::options::Options;
use std::{
	cell::RefCell, collections::HashMap, collections::HashSet, rc::Rc,
};

pub fn load_node<T, U>(
	nodes: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
	path: &String,
	read_from_file: fn(&String) -> U,
	create_node: fn(&String, U) -> Rc<RefCell<Node<T>>>,
) -> Rc<RefCell<Node<T>>> {
	let clean_path = path
		.replace("../", "")
		.replace("..\\", "")
		.replace("./", "")
		.replace(".\\", "");
	if nodes.contains_key(&clean_path) == false {
		let dm = read_from_file(&clean_path);
		let new_node = create_node(&clean_path, dm);
		nodes.insert(clean_path.clone(), new_node.clone());
	}
	nodes[&clean_path].clone()
}

/// Build directed acyclic graph from nodes
pub fn build_dag_from_nodes<T, U>(
	node: Rc<RefCell<Node<T>>>,
	nodes: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
	pbranch: &mut HashSet<String>,
	sbranch: &mut HashSet<String>,
	read_from_file: fn(&String) -> U,
	create_node: fn(&String, U) -> Rc<RefCell<Node<T>>>,
	options: &Options,
	sdepth: i64,
) {
	let node_path = node.borrow().path.clone();

	// Add successors
	if sdepth != 0 {
		sbranch.insert(node_path.clone());
		let incl_list = node.borrow().incl.clone();
		for incl_path in incl_list.iter() {
			let incl_node =
				load_node(nodes, incl_path, read_from_file, create_node);
			// These two conditions are required to guarantee termination
			let already_in_dag =
				incl_node.borrow().has_predecessor(node.clone());
			let cycle = sbranch.contains(incl_path);
			if cycle == false {
				if already_in_dag == false {
					incl_node.borrow_mut().add_predecessor_node(node.clone());
					nodes[&"//".to_string()]
						.borrow_mut()
						.add_predecessor_node(incl_node.clone());
					build_dag_from_nodes(
						incl_node.clone(),
						nodes,
						&mut HashSet::new(),
						sbranch,
						read_from_file,
						create_node,
						options,
						if sdepth > 0 { sdepth - 1 } else { sdepth },
					);
				}
			}
		}
		sbranch.remove(&node_path);
	}

	// Add predecessors
	pbranch.insert(node_path.clone());
	let req_list = node.borrow().req.clone();
	for req_path in req_list.iter() {
		let req_node =
			load_node(nodes, req_path, read_from_file, create_node);
		// These two conditions are required to guarantee termination
		let already_in_dag =
			node.borrow().has_predecessor(req_node.clone());
		let cycle = pbranch.contains(req_path);
		if cycle == false {
			if already_in_dag == false {
				node.borrow_mut().add_predecessor_node(req_node.clone());
				build_dag_from_nodes(
					req_node.clone(),
					nodes,
					pbranch,
					&mut HashSet::new(),
					read_from_file,
					create_node,
					options,
					if sdepth > 0 { sdepth - 1 } else { sdepth },
				);
			}
		}
	}
	pbranch.remove(&node_path);
}

pub fn remove_indirect_predecessors<T>(node: Rc<RefCell<Node<T>>>) {
	let mut remove = vec![];
	for child in node.borrow().predecessors().iter() {
		for grandchild in child.borrow().predecessors().iter() {
			let index =
				node.borrow().get_predecessor_index(grandchild.clone());
			if index.is_ok() {
				remove.push(index.unwrap());
			}
		}
	}
	remove.sort();
	remove.dedup();
	let children_form_cycle =
		{ remove.len() == node.borrow().predecessors().len() };
	let terminal_index = if children_form_cycle { 1 } else { 0 };
	for &i in remove.iter().rev() {
		if i >= terminal_index {
			node.borrow_mut().remove_predecessor_by_index(i);
		}
	}
}

/// Modified Depth First Search; does not "discover" a node until all
/// branches leading to that node have been traversed; sorting branches
/// in the DAG will affect the output; recommended to use
/// `Node::sort_predecessor_branches()` method on root node before
/// sorting
pub fn topological_sort<T>(
	node: Rc<RefCell<Node<T>>>
) -> Vec<Rc<RefCell<Node<T>>>> {
	let mut sorted_nodes = vec![];
	if node.borrow().sorted == false {
		node.borrow_mut().sorted = true;
		sorted_nodes.push(node.clone());
	}
	let mut stack = vec![node.clone()];
	while stack.is_empty() == false {
		let v = stack.pop().unwrap();
		// Use <= instead of < to ensure that the root node (with zero
		// successors) is visited; otherwise, no nodes will be added to the
		// list of sorted nodes
		if v.borrow().times_visited <= v.borrow().num_successors() {
			// Iterative DFS for a DAG, but node is considered
			// visited/discovered only if all of its parents have been
			// visited/discovered; the >= condition ensures that a root with
			// no successors is never added
			v.borrow_mut().incr_times_visited();
			// FIXME: nodes with deadlines don't know how many more
			// successors have to be visited before they get added
			if v.borrow().times_visited >= v.borrow().num_successors() {
				for w in v.borrow().predecessors().iter() {
					stack.push(w.clone());
				}
			}

			if v.borrow().is_discovered() == true {
				if v.borrow().sorted == false {
					v.borrow_mut().sorted = true;
					sorted_nodes.push(v.clone());
				}
			}
		}
	}
	sorted_nodes
}
