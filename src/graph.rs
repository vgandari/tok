use crate::node::Node;
use std::{
	cell::RefCell, collections::HashMap, collections::HashSet, rc::Rc,
};

pub fn load_node<T, U>(
	nodes: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
	path: &String,
	read_from_file: fn(&String) -> U,
	create_node: fn(&String, U) -> Rc<RefCell<Node<T>>>,
) {
	if nodes.contains_key(path) == false {
		println!("Reading {}", path);
		let dm = read_from_file(&path);
		let new_node = create_node(&path, dm);
		nodes.insert(path.clone(), new_node.clone());
	}
}

/// Build directed acyclic graph from nodes
pub fn build_dag_from_nodes<T, U>(
	node: Rc<RefCell<Node<T>>>,
	nodes: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
	pbranch: &mut HashSet<String>,
	sbranch: &mut HashSet<String>,
	read_from_file: fn(&String) -> U,
	create_node: fn(&String, U) -> Rc<RefCell<Node<T>>>,
) {
	build_dag_backward(
		node.clone(),
		nodes,
		pbranch,
		sbranch,
		read_from_file,
		create_node,
	);
}

/// Build directed acyclic graph from nodes, starting at root; `nodes`
/// must contain a node with "//" as a value
fn build_dag_backward<T, U>(
	node: Rc<RefCell<Node<T>>>,
	nodes: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
	pbranch: &mut HashSet<String>,
	sbranch: &mut HashSet<String>,
	read_from_file: fn(&String) -> U,
	create_node: fn(&String, U) -> Rc<RefCell<Node<T>>>,
) {
	let node_path = { node.borrow().path.clone() };
	pbranch.insert(node_path.clone());

	let paths = node.borrow().after.clone();
	for dirty_path in paths {
		let path = dirty_path.replace("../", "").replace("./", "");
		let cycle = pbranch.contains(&path);
		if cycle == false {
			load_node(nodes, &path, read_from_file, create_node);
			let predecessor = nodes[&path].clone();
			let num_predecessors = node.borrow().predecessors().len();
			{
				if predecessor.borrow().has_predecessor(node.clone()) == false {
					node.borrow_mut().add_predecessor_node(predecessor.clone());
				}
			}
			let new_predecessor_is_duplicate =
				!(num_predecessors < node.borrow().predecessors().len());
			if new_predecessor_is_duplicate == false {
				predecessor.borrow_mut().incr_num_successors();
			}
			build_dag_forward(
				predecessor.clone(),
				nodes,
				pbranch,
				sbranch,
				read_from_file,
				create_node,
			);
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
				let i = index.unwrap();
				remove.push(i);
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
			let child = node.borrow().predecessors()[i].clone();
			child.borrow_mut().decr_num_successors();
			node.borrow_mut().remove_predecessor_by_index(i);
		}
	}
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

/// Build directed acyclic graph from nodes, starting at leaf; `nodes`
/// must contain a node with "//" as a value
fn build_dag_forward<T, U>(
	leaf: Rc<RefCell<Node<T>>>,
	nodes: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
	pbranch: &mut HashSet<String>,
	sbranch: &mut HashSet<String>,
	read_from_file: fn(&String) -> U,
	create_node: fn(&String, U) -> Rc<RefCell<Node<T>>>,
) {
	let leaf_path = { leaf.borrow().path.clone() };
	sbranch.insert(leaf_path.clone());
	let root = nodes[&"//".to_string()].clone();
	let paths = leaf.borrow().before.clone();

	for dirty_path in paths {
		let path = dirty_path.replace("../", "").replace("./", "");
		let cycle = sbranch.contains(&path);
		if cycle == false {
			load_node(nodes, &path, read_from_file, create_node);
			let successor = nodes[&path].clone();
			let num_predecessors = successor.borrow().predecessors().len();
			{
				if leaf.borrow().has_predecessor(successor.clone()) == false {
					root.borrow_mut().add_predecessor_node(successor.clone());
					successor.borrow_mut().add_predecessor_node(leaf.clone());
				}
			}
			let new_predecessor_is_duplicate =
				!(num_predecessors < successor.borrow().predecessors().len());
			if new_predecessor_is_duplicate == false {
				successor.borrow_mut().incr_num_successors();
			}
			build_dag_forward(
				successor.clone(),
				nodes,
				pbranch,
				sbranch,
				read_from_file,
				create_node,
			);
		}
	}
	build_dag_backward(
		leaf.clone(),
		nodes,
		pbranch,
		sbranch,
		read_from_file,
		create_node,
	);
	sbranch.remove(&leaf_path);
}
