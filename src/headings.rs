use crate::node::Node;
use crate::topic::Topic;
use std::{cell::RefCell, rc::Rc};

/// Compute minimum cost for a node to be considered the end of a
/// section with deepest heading level
pub fn compute_min_dag_costs(sorted_costs: Vec<usize>) -> usize {
	let mut rank = vec![0; sorted_costs.len()];
	let mut j = 0;

	// rank DAG costs
	for i in 1..sorted_costs.len() {
		j += 1;
		if sorted_costs[i - 1] < sorted_costs[i] {
			rank[i] = j;
		} else {
			rank[i] = rank[i - 1];
		}
	}

	// Add minimum cost to ranks
	let min_cost_index = if sorted_costs.len() >= 6 {
		sorted_costs.len() - 6
	} else {
		sorted_costs.len() - 1
	};
	for i in 0..rank.len() {
		rank[i] += sorted_costs[min_cost_index];
	}

	// Get cost corresponding to h-index
	let mut min_cost = 0;
	let mut i = 0;
	'c: for cost in sorted_costs {
		if cost >= rank[i] {
			min_cost = cost;
			break 'c;
		}
		i += 1;
	}
	min_cost
}

/// Set heading depth for node and all its predecessors if generating
/// headings
pub fn set_heading_depth(
	node: Rc<RefCell<Node<Topic>>>,
	min_cost: &usize,
) {
	for p in node.borrow().predecessors() {
		let possibly_parallel = {
			p.borrow().dag_cost() > *min_cost
				&& node.borrow().predecessors().len() > 1
		};
		if possibly_parallel {
			p.borrow_mut().data_mut().heading_depth =
				node.borrow().data().heading_depth + 1;
		}
		set_heading_depth(p.clone(), min_cost);
	}
}

/// Find heading repeated heading depths; e.g. if "1" appears twice and
/// max heading depth is 2, set flag to create section headings
// pub fn set_flags_to_create_headings(
// 	sorted_nodes_back_to_front: &Vec<Rc<RefCell<Node<Topic>>>>
// ) {
// 	let mut generate_headings = vec![false; 7];
// 	generate_headings[0] = true;
// 	for current_depth in 1..6 {
// 		if generate_headings[current_depth - 1] == true {
// 			let mut gen = false;
// 			for n in sorted_nodes_back_to_front {
// 				if n.borrow().heading_depth == current_depth && gen == true {
// 					generate_headings[current_depth] = true;
// 				}
// 				if n.borrow().heading_depth == current_depth {
// 					gen = true;
// 				}
// 			}
// 		}
// 	}
// }

/// Ensure that if there is a chapter, then there are at least two
/// chapters; if there is a section within a chapter, there are at least
/// two sections within a chapter, etc.
// TODO: call from main
// pub fn remove_unrepeated_heading_depths<T>(
// 	i: usize,
// 	sorted_nodes: &Vec<Rc<RefCell<Node<T>>>>,
// ) {
// 	if sorted_nodes.len() > i {
// 		let A = sorted_nodes[i].clone();
// 		if A.borrow().heading_depth > 0 {
// 			let mut generate_headings = false;
// 			'search_this_level_or_deeper: for j in i..sorted_nodes.len() {
// 				let B = sorted_nodes[j].clone();
// 				// heading depth is repeated => generate headings for this depth
// 				if B.borrow().heading_depth == A.borrow().heading_depth {
// 					generate_headings = true;
// 				}
// 				// finished searching for another node with this heading depth
// 				if B.borrow().heading_depth > 0
// 					&& B.borrow().heading_depth < A.borrow().heading_depth
// 				{
// 					if generate_headings == false {
// 						// Clear heading depth for nodes with this heading depth and deeper
// 						for k in i..j {
// 							let C = sorted_nodes[k].clone();
// 							C.borrow_mut().heading_depth = 0;
// 						}
// 						break 'search_this_level_or_deeper;
// 					}
// 				}
// 				// FIXME: Move
// 				// Do generate headings for this heading depth
// 				// Generate headings one level deeper
// 				if B.borrow().heading_depth > A.borrow().heading_depth {
// 					remove_unrepeated_heading_depths(j, sorted_nodes);
// 				}
// 				remove_unrepeated_heading_depths(j, sorted_nodes);
// 			}
// 		} else {
// 			remove_unrepeated_heading_depths(i + 1, sorted_nodes);
// 		}
// 	}
// }

/// Add to nodes the heading titles that will show up in final document
pub fn add_heading_titles_to_nodes(
	sorted_nodes: &Vec<Rc<RefCell<Node<Topic>>>>
) {
	for chd in 1..6 {
		let mut ht = "".to_string();
		let mut prev_node: Option<Rc<RefCell<Node<Topic>>>> = None;
		for current_node in sorted_nodes {
			let hd = current_node.borrow().data().heading_depth;

			// TODO: make a data_mut method for topic
			if prev_node.is_some() && hd <= chd && hd > 0 {
				let p = prev_node.unwrap().clone();
				p.borrow_mut().data_mut().heading_titles.push(ht.clone());
			}

			if hd == chd {
				ht = current_node.borrow().data().label.clone();
			} else if hd < chd && hd > 0 {
				ht = "".to_string();
			}

			prev_node = Some(current_node.clone());
		}
		prev_node
			.unwrap()
			.borrow_mut()
			.data_mut()
			.heading_titles
			.push(ht.clone());
	}
}
