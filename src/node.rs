use std::{cell::RefCell, rc::Rc};

/// Node info for constructing tree
pub struct Node<T> {
	/// Flag for topological sort
	discovered: bool,
	/// Cost of tree rooted at this node; used for sorting branches
	dag_cost: usize,
	/// Number of successors (used for breaking cycles in topological
	/// sort)
	num_successors: usize,
	pub heading_depth: usize,
	/// Number of predecessors (used for inserting headings)
	// num_predecessors: usize,
	/// Cost of this node; used for computing tree cost; uses length of
	/// text string as heuristic for how long it takes to master the
	/// content provided in this node
	pub cost: usize,
	/// YAML key; Path to corresponding YAML file; also used as reflabel in LaTeX
	pub path: String,
	/// Sequence of file paths with node data that this node must come
	/// after; relationship may be broken if tok detects cycles
	pub after: Vec<String>,
	/// Sequence of file paths with node data that this node must come
	/// before; relationship may be broken if tok detects cycles
	pub before: Vec<String>,
	/// Vector of pointers to predecessor nodes; necessary for
	/// constructing tree; not a YAML key
	predecessors: Vec<Rc<RefCell<Node<T>>>>,
	/// Vector of pointers to predecessor nodes; necessary for
	/// constructing tree; not a YAML key
	successors: Vec<Rc<RefCell<Node<T>>>>,
	/// Data contained in this node
	data: T,
}

impl<T> PartialEq for Node<T> {
	fn eq(
		&self,
		other: &Node<T>,
	) -> bool {
		self.path == other.path
	}
}

impl<T> Node<T> {
	/// Construct instance with shared reference
	pub fn new(
		filename: &String,
		data: T,
	) -> Rc<RefCell<Node<T>>> {
		Rc::new(RefCell::new(Node::<T> {
			path: String::from(filename.clone()),
			predecessors: vec![],
			successors: vec![],
			after: vec![],
			before: vec![],
			num_successors: 0,
			heading_depth: 0,
			discovered: false,
			data: data,
			dag_cost: 1,
			cost: 1,
		}))
	}

	/// Add a predecessor to this node; also updates the predecessor's
	/// number of successors
	pub fn add_predecessor_node(
		&mut self,
		predecessor: Rc<RefCell<Node<T>>>,
	) {
		// Get number of predecessors
		// let num_predecessors = self.predecessors.len();

		// Add predecessor
		self.predecessors.push(predecessor.clone());
		self.dedup_predecessors();
		// let new_predecessor_is_duplicate =
		// 	!(num_predecessors < self.predecessors.len());

		// Ensure additional predecessor is not a duplicate
		// if new_predecessor_is_duplicate == false {
		// predecessor.borrow_mut().incr_num_successors();
		// 	let update = predecessor.borrow().dag_cost();
		// 	self.add_to_dag_cost(update);
		// }
	}

	/// Store data in node
	pub fn set_data(
		&mut self,
		data: T,
	) {
		self.data = data;
	}

	/// Check if node is discovered; used in topological sort
	pub fn is_discovered(&self) -> bool {
		self.discovered
	}

	/// Mark node as discovered; used in topological sort
	pub fn mark_discovered(&mut self) {
		self.discovered = true;
	}

	pub fn num_successors(&self) -> usize {
		self.num_successors
	}

	/// Increment number of successors
	pub fn incr_num_successors(&mut self) {
		self.num_successors += 1;
	}

	/// Decrement number of successors
	pub fn decr_num_successors(&mut self) {
		if self.has_multiple_successors() {
			self.num_successors -= 1;
		}
	}

	/// Check if node has single successor
	pub fn has_single_successor(&self) -> bool {
		self.num_successors == 1
	}

	/// Check if node has more than one successor
	pub fn has_multiple_successors(&self) -> bool {
		self.num_successors > 1
	}

	pub fn dedup_predecessors(&mut self) {
		self
			.predecessors
			.sort_by(|a, b| a.borrow().path.cmp(&b.borrow().path));
		self.predecessors.dedup();
	}

	pub fn predecessors(&self) -> Vec<Rc<RefCell<Node<T>>>> {
		self.predecessors.clone()
	}

	pub fn successors(&self) -> Vec<Rc<RefCell<Node<T>>>> {
		self.successors.clone()
	}

	pub fn dag_cost(&self) -> usize {
		self.dag_cost
	}

	pub fn set_dag_cost(
		&mut self,
		dag_cost: usize,
	) {
		self.dag_cost = dag_cost;
	}

	pub fn data(&self) -> &T {
		&self.data
	}

	pub fn dedup_after(&mut self) {
		self.after.sort_unstable();
		self.after.dedup();
	}

	pub fn dedup_before(&mut self) {
		self.before.sort_unstable();
		self.before.dedup();
	}

	/// Find index of predecessor node; returns zero if there are no
	/// predecessors
	pub fn get_predecessor_index(
		&self,
		pred: Rc<RefCell<Node<T>>>,
	) -> Result<usize, ()> {
		for i in 0..self.predecessors.len() {
			if self.predecessors[i].borrow().path == pred.borrow().path {
				return Ok(i);
			}
		}
		Err(())
	}

	/// Check if `pred` is a predecessor of this node; returns false if
	/// this node has no predecessors
	pub fn has_predecessor(
		&self,
		pred: Rc<RefCell<Node<T>>>,
	) -> bool {
		self.get_predecessor_index(pred.clone()).is_ok()
	}

	/// Remove predecessor node; does nothing if `pred` is not a predecessor
	pub fn remove_predecessor(
		&mut self,
		pred: Rc<RefCell<Node<T>>>,
	) {
		let index = self.get_predecessor_index(pred.clone());
		if index.is_ok() {
			self.predecessors.remove(index.unwrap());
		}
	}

	/// Remove predecessor node, given its index; paniccs if `index` is
	/// out of bounds
	pub fn remove_predecessor_by_index(
		&mut self,
		index: usize,
	) {
		if index < self.predecessors().len()
			|| self.predecessors().len() == 0
		{
			self.predecessors.remove(index);
		}
	}

	/// Compute cost of graph with this node as root; ignores cycles;
	/// required for sorting branches;
	pub fn compute_dag_cost(&mut self) -> usize {
		for it in self.predecessors.iter() {
			// There are still cycles that we need to ignore
			let cycle = { it.try_borrow_mut().is_err() };
			if cycle == false {
				self.dag_cost += it.borrow_mut().compute_dag_cost();
			}
		}
		self.dag_cost
	}

	/// Sort predecessors by tree cost so that shorter branches appear
	/// before/after longer branches, depending on option selected
	pub fn sort_predecessor_branches(
		&mut self,
		reverse: bool,
	) {
		match reverse {
			true => {
				self.predecessors.sort_by(|a, b| {
					a.borrow().dag_cost.cmp(&b.borrow().dag_cost)
				});
			}
			false => {
				self.predecessors.sort_by(|b, a| {
					a.borrow().dag_cost.cmp(&b.borrow().dag_cost)
				});
			}
		}
	}
}
