use crate::node::Node;
use std::{cell::RefCell, rc::Rc};

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
	for i in 0..rank.len() {
		rank[i] += sorted_costs[0];
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

pub fn set_heading_depth<T>(
	node: Rc<RefCell<Node<T>>>,
	min_cost: &usize,
) {
	for p in node.borrow().predecessors() {
		let possible_end_of_section = {
			p.borrow().dag_cost() > *min_cost
				&& node.borrow().predecessors().len() > 1
		};
		if possible_end_of_section {
			p.borrow_mut().heading_depth = node.borrow().heading_depth + 1;
		}
		set_heading_depth(p.clone(), min_cost);
	}
}
