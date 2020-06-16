use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Node info for constructing tree
pub struct Node<T> {
	/// Flag for topological sort
	discovered: bool,
	/// Cost of tree rooted at this node; used for sorting branches
	tree_cost: u64,
	/// Number of successors (used for breaking cycles in topological
	/// sort)
	num_successors: u64,
	/// Number of predecessors (used for inserting headings)
	// num_predecessors: u64,
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

	pub fn num_predecessors_fast(&self) -> usize {
		self.predecessors.len()
	}

	pub fn num_successors(&mut self) -> usize {
		self.dedup_successors();
		self.successors.len()
	}

	pub fn dedup_predecessors(&mut self) {
		self
			.predecessors
			.sort_by(|a, b| a.borrow().path.cmp(&b.borrow().path));
		self.predecessors.dedup();
	}

	pub fn dedup_successors(&mut self) {
		self
			.successors
			.sort_by(|a, b| a.borrow().path.cmp(&b.borrow().path));
		self.successors.dedup();
	}

	pub fn push_successor(
		&mut self,
		succ: Rc<RefCell<Node<T>>>,
	) {
		self.successors.push(succ);
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
		self.after.sort_unstable();
		self.after.dedup();
	}

	pub fn dedup_before(&mut self) {
		self.before.sort_unstable();
		self.before.dedup();
	}

	pub fn check_for_predecessor(
		&self,
		pred: Rc<RefCell<Node<T>>>,
	) -> usize {
		for i in 0..self.predecessors.len() {
			if self.predecessors[i] == pred {
				return i;
			}
		}
		self.predecessors.len()
	}

	pub fn check_for_successor(
		&self,
		succ: Rc<RefCell<Node<T>>>,
	) -> usize {
		println!("Checking for successors");
		for i in 0..self.successors.len() {
			if self.successors[i] == succ {
				println!("Found {}", succ.borrow().path);
				return i;
			}
		}
		self.successors.len()
	}

	pub fn has_predecessor(
		&self,
		pred: Rc<RefCell<Node<T>>>,
	) -> bool {
		let index = self.check_for_predecessor(pred.clone());
		if index < self.predecessors.len() {
			true
		} else {
			false
		}
	}

	pub fn has_successor(
		&self,
		succ: Rc<RefCell<Node<T>>>,
	) -> bool {
		let index = self.check_for_predecessor(succ.clone());
		if index < self.successors.len() {
			true
		} else {
			false
		}
	}

	/// Remove predecessor node; does nothing if `pred` is not a predecessor
	pub fn remove_predecessor(
		&mut self,
		pred: Rc<RefCell<Node<T>>>,
	) {
		let index = self.check_for_predecessor(pred.clone());
		if index < self.predecessors.len() {
			self.predecessors.remove(index);
		}
	}

	pub fn remove_successor(
		&mut self,
		succ: Rc<RefCell<Node<T>>>,
	) {
		let index = self.check_for_successor(succ.clone());
		if index < self.successors.len() {
			self.successors.remove(index);
		}
	}

	/// Compute cost of tree with this node as root; required for sorting
	/// branches
	pub fn compute_tree_cost(&mut self) -> u64 {
		for it in self.predecessors.iter() {
			let valid_borrow = { it.try_borrow_mut().is_ok() };
			if valid_borrow == true {
				self.tree_cost += it.borrow_mut().compute_tree_cost();
			}
		}
		self.tree_cost
	}

	/// Sort predecessors by tree cost so that shorter branches appear
	/// before longer branches if possible
	pub fn sort_predecessor_branches(
		&mut self,
		reverse: bool,
	) {
		// for it in self.predecessors.iter() {
		//   println!("{}", it.borrow().path);
		//   let valid_borrow = { it.try_borrow_mut().is_ok() };
		//   if valid_borrow == true {
		//     it.borrow_mut().sort_predecessor_branches(reverse);
		//   }
		// }
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

pub fn load<T, U>(
	nodes: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
	path: &String,
	read_from_file: fn(&String) -> U,
	create_node: fn(&String, U) -> Rc<RefCell<Node<T>>>,
) {
	let clean_path = path.replace("../", "").replace("./", "");
	if nodes.contains_key(&clean_path) == false {
		let dm = read_from_file(&clean_path);
		let new_node = create_node(&clean_path, dm);
		nodes.insert(clean_path, new_node.clone());
	}
}

pub fn add_predecessor_node<T>(
	node: Rc<RefCell<Node<T>>>,
	predecessor: Rc<RefCell<Node<T>>>,
) {
	// Add predecessor
	node.borrow_mut().push_predecessor(predecessor.clone());

	// Get number of predecessors
	let num_predecessors = node.borrow_mut().num_predecessors();

	// Add predecessor
	node.borrow_mut().push_predecessor(predecessor.clone());

	// Ensure additional predecessor is not a duplicate
	if num_predecessors < node.borrow_mut().num_predecessors() {
		predecessor.borrow_mut().incr_num_successors();
		let update = predecessor.borrow().tree_cost();
		node.borrow_mut().add_to_tree_cost(update);
	}
}

/// Add successor node to tree;
/// root and node must not refer to the same object
pub fn add_successor_node<T>(
	root: Rc<RefCell<Node<T>>>,
	node: Rc<RefCell<Node<T>>>,
	successor: Rc<RefCell<Node<T>>>,
) {
	add_predecessor_node(root.clone(), successor.clone());
	add_predecessor_node(successor.clone(), node.clone());
}
