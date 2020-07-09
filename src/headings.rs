use crate::node::Node;
use crate::topic::Topic;
use std::{cell::RefCell, rc::Rc};

/// Compute minimum cost for a node to be considered the end of a
/// section with deepest heading level
pub fn compute_min_dag_costs(
	extra_headings: bool,
	sorted_costs: Vec<usize>,
) -> usize {
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
	if extra_headings == false {
		for i in 0..rank.len() {
			rank[i] += sorted_costs[min_cost_index];
		}
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
	for search_heading_depth in 1..7 {
		let mut heading_title = "".to_string();
		let mut prev_node: Option<Rc<RefCell<Node<Topic>>>> = None;
		for current_node in sorted_nodes {
			let current_heading_depth =
				current_node.borrow().data().heading_depth;

			// Criteria for ending a section with the current heading depth
			let current_node_is_not_first_node = prev_node.is_some();
			let search_heading_depth_is_deep_enough =
				current_heading_depth <= search_heading_depth;
			let current_node_ends_section = current_heading_depth > 0;

			// Append heading title to previous node; the document starts at
			// the back of the sorted list and ends at the front, so the
			// previous node appears later in the generated document; the
			// previous node starts a new section and the title for that
			// section is inserted into that node; the current node ends a
			// different section
			if current_node_is_not_first_node
				&& search_heading_depth_is_deep_enough
				&& current_node_ends_section
			{
				let p = prev_node.unwrap().clone();
				p.borrow_mut()
					.data_mut()
					.heading_titles
					.push(heading_title.clone());
				p.borrow_mut().data_mut().heading_depth_start =
					current_heading_depth;
			}

			// Criteria for starting a section with the current heading depth
			let section_with_same_depth_found =
				current_heading_depth == search_heading_depth;

			// Get title for previous section from node with same heading
			// depth
			if section_with_same_depth_found {
				heading_title = current_node.borrow().data().label.clone();
			} else if current_heading_depth < search_heading_depth
				&& current_heading_depth > 0
			{
				heading_title = "".to_string();
			}

			prev_node = Some(current_node.clone());
		}

		// Append heading title to node at back of sorted nodes list; This
		// will be the title of the first chapter/section/etc. (depending on
		// maximum heading depth) of the document
		let p = prev_node.unwrap().clone();
		p.borrow_mut()
			.data_mut()
			.heading_titles
			.push(heading_title.clone());
		p.borrow_mut().data_mut().heading_depth_start = 1;
	}
}
