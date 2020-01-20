use std::{cell::RefCell, rc::Rc};

/// Compile PDF from generated tex file "zzz.tex"
pub struct Node<T> {
	/// Path to corresponding YAML file; also used as reflabel in LaTeX
	pub path: String,
	/// Vector of pointers to predecessor nodes; necessary for
	/// constructing tree
	pub predecessors: Vec<Rc<RefCell<Node<T>>>>,
	pub after: Vec<String>,
	pub before: Vec<String>,
	/// Cost of this node; used for computing tree cost; uses length of
	/// text string as heuristic for how long it takes to master the
	/// content provided in this node
	pub node_cost: u64,
	/// Cost of tree rooted at this node; used for sorting branches
	pub tree_cost: u64,

	/// Number of successors (used for breaking cycles in Depth First
	/// Search/Sort)
	num_successors: u64,
	/// Data contained in this node
	pub data: T,
	pub discovered: bool,
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
			after: vec![],
			before: vec![],
			num_successors: 0,
			discovered: false,
			data: data,
			tree_cost: 1,
			node_cost: 1,
		}))
	}

	/// Decrement number of successors
	pub fn decr_num_successors(&mut self) {
		self.num_successors -= 1;
	}

	/// Increment number of successors
	pub fn incr_num_successors(&mut self) {
		self.num_successors += 1;
	}
	pub fn has_single_successor(&self) -> bool {
		self.num_successors == 1
	}

	pub fn has_multiple_successors(&self) -> bool {
		self.num_successors > 1
	}

	pub fn num_predecessors(&mut self) -> usize {
		self.dedup_predecessors();
		self.predecessors.len()
	}

	pub fn dedup_predecessors(&mut self) {
		self
			.predecessors
			.sort_by(|a, b| a.borrow().path.cmp(&b.borrow().path));
		self.predecessors.dedup();
	}

	pub fn dedup_after(&mut self) {
		self.after.sort();
		self.after.dedup();
	}

	pub fn dedup_before(&mut self) {
		self.before.sort();
		self.before.dedup();
	}

	/// Compute cost of tree with this node as root; required for sorting
	/// branches
	pub fn compute_tree_cost(&mut self) -> u64 {
		// Remove duplicates before computing costs
		self.dedup_predecessors();
		// Compute tree costs to sort branches
		for it in self.predecessors.iter() {
			self.tree_cost += it.borrow_mut().compute_tree_cost();
		}
		self.tree_cost
	}

	/// Sort predecessors by tree cost so that shorter branches appear
	/// before longer branches if possible
	pub fn sort_predecessor_branches(
		&mut self,
		reverse: bool,
	) {
		for it in self.predecessors.iter() {
			it.borrow_mut().sort_predecessor_branches(reverse);
		}
		match reverse {
			true => {
				self.predecessors.sort_by(|a, b| {
					a.borrow().tree_cost.cmp(&b.borrow().tree_cost)
				});
			}
			false => {
				self.predecessors.sort_by(|b, a| {
					a.borrow().tree_cost.cmp(&b.borrow().tree_cost)
				});
			}
		}
	}
}
