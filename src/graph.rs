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
	sdepth: i64,
) {
	build_dag_backward(
		node.clone(),
		nodes,
		pbranch,
		sbranch,
		read_from_file,
		create_node,
		sdepth,
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
	sdepth: i64,
) {
	let node_path = { node.borrow().path.clone() };
	pbranch.insert(node_path.clone());

	let paths = node.borrow().req.clone();
	for dirty_path in paths {
		let path = dirty_path.replace("../", "").replace("./", "");
		let cycle = pbranch.contains(&path);
		if cycle == false {
			load_node(nodes, &path, read_from_file, create_node);
			let predecessor = nodes[&path].clone();
			{
				if predecessor.borrow().has_predecessor(node.clone()) == false {
					node.borrow_mut().add_predecessor_node(predecessor.clone());
				}
			}
			build_dag_forward(
				predecessor.clone(),
				nodes,
				pbranch,
				sbranch,
				read_from_file,
				create_node,
				if sdepth > 0 { sdepth - 1 } else { sdepth },
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

/// Build directed acyclic graph from nodes, starting at leaf; `nodes`
/// must contain a node with "//" as a value
fn build_dag_forward<T, U>(
	leaf: Rc<RefCell<Node<T>>>,
	nodes: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
	pbranch: &mut HashSet<String>,
	sbranch: &mut HashSet<String>,
	read_from_file: fn(&String) -> U,
	create_node: fn(&String, U) -> Rc<RefCell<Node<T>>>,
	sdepth: i64,
) {
	let leaf_path = { leaf.borrow().path.clone() };
	sbranch.insert(leaf_path.clone());
	if sdepth != 0 {
		let root = nodes[&"//".to_string()].clone();
		let paths = leaf.borrow().incl.clone();

		for dirty_path in paths {
			let path = dirty_path.replace("../", "").replace("./", "");
			let cycle = sbranch.contains(&path);
			if cycle == false {
				load_node(nodes, &path, read_from_file, create_node);
				let successor = nodes[&path].clone();
				{
					if leaf.borrow().has_predecessor(successor.clone()) == false {
						root.borrow_mut().add_predecessor_node(successor.clone());
						successor.borrow_mut().add_predecessor_node(leaf.clone());
					}
				}
				build_dag_forward(
					successor.clone(),
					nodes,
					pbranch,
					sbranch,
					read_from_file,
					create_node,
					if sdepth > 0 { sdepth - 1 } else { sdepth },
				);
			}
		}
	}
	build_dag_backward(
		leaf.clone(),
		nodes,
		pbranch,
		sbranch,
		read_from_file,
		create_node,
		sdepth,
	);
	sbranch.remove(&leaf_path);
}
