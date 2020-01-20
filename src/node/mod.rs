use std::{cell::RefCell, rc::Rc};

/// Compile PDF from generated tex file "zzz.tex"
pub struct Node<T> {
	/// Flag for topological sort
	discovered: bool,
	/// Cost of tree rooted at this node; used for sorting branches
	tree_cost: u64,
	/// Number of successors (used for breaking cycles in topological
	/// sort)
	num_successors: u64,
	/// Cost of this node; used for computing tree cost; uses length of
	/// text string as heuristic for how long it takes to master the
	/// content provided in this node
	pub cost: u64,
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
			after: vec![],
			before: vec![],
			num_successors: 0,
			discovered: false,
			data: data,
			tree_cost: 1,
			cost: 1,
		}))
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

	pub fn push_predecessor(
		&mut self,
		pred_ref: Rc<RefCell<Node<T>>>,
	) {
		self.predecessors.push(pred_ref);
	}
	pub fn predecessors(&self) -> Vec<Rc<RefCell<Node<T>>>> {
		self.predecessors.clone()
	}

	pub fn tree_cost(&self) -> u64 {
		self.tree_cost
	}

	pub fn set_tree_cost(
		&mut self,
		tree_cost: u64,
	) {
		self.tree_cost = tree_cost;
	}

	pub fn data(&self) -> &T {
		&self.data
	}

	pub fn add_to_tree_cost(
		&mut self,
		update: u64,
	) {
		self.tree_cost += update;
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
