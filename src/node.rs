use std::{cell::RefCell, cmp::Ordering, rc::Rc};
type NodeComparison<T> = fn(
	reverse: bool,
	&Rc<RefCell<Node<T>>>,
	&Rc<RefCell<Node<T>>>,
) -> Ordering;

/// Node info for constructing tree
pub struct Node<T> {
	/// Whether or not this node has been sorted for document
	pub sorted: bool,
	/// Cost of tree rooted at this node; used for sorting branches
	dag_cost: usize,
	/// Number of successors (used for breaking cycles in topological
	/// sort)
	num_successors: usize,
	/// Cost of this node; used for computing tree cost; uses length of
	/// text string as heuristic for how long it takes to master the
	/// content provided in this node
	pub cost: usize,
	/// YAML key; Path to corresponding YAML file; also used as reflabel in LaTeX
	pub path: String,
	/// Sequence of file paths with node data that this node must come
	/// after; relationship may be broken if tok detects cycles
	pub req: Vec<String>,
	/// Sequence of file paths with node data that this node must come
	/// before; relationship may be broken if tok detects cycles
	pub incl: Vec<String>,
	/// Vector of pointers to predecessor nodes; necessary for
	/// constructing tree; not a YAML key
	predecessors: Vec<Rc<RefCell<Node<T>>>>,
	/// Vector of pointers to predecessor nodes; necessary for
	/// constructing tree; not a YAML key
	successors: Vec<Rc<RefCell<Node<T>>>>,
	/// Data contained in this node
	data: T,
	pub times_visited: usize,
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
			sorted: false,
			path: String::from(filename.clone()),
			predecessors: vec![],
			successors: vec![],
			req: vec![],
			incl: vec![],
			num_successors: 0,
			data: data,
			dag_cost: 1,
			cost: 1,
			times_visited: 0,
		}))
	}

	/// Add a predecessor to this node; also updates the predecessor's
	/// number of successors
	pub fn add_predecessor_node(
		&mut self,
		predecessor: Rc<RefCell<Node<T>>>,
	) {
		// Get number of predecessors
		let num_predecessors = self.predecessors.len();

		// Add predecessor
		self.predecessors.push(predecessor.clone());
		self.dedup_predecessors();
		let new_predecessor_is_duplicate =
			!(num_predecessors < self.predecessors.len());

		// Ensure additional predecessor is not a duplicate
		if new_predecessor_is_duplicate == false {
			predecessor.borrow_mut().incr_num_successors();
		}
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
		self.times_visited == self.num_successors
	}

	pub fn num_predecessors(&self) -> usize {
		self.predecessors.len()
	}

	pub fn reset(&mut self) {
		self.num_successors = 0;
		self.times_visited = 0;
		self.dag_cost = 0;
		for n in self.predecessors.iter() {
			n.borrow_mut().reset();
		}
		self.predecessors.clear();
	}

	pub fn num_successors(&self) -> usize {
		self.num_successors
	}

	/// Increment number of successors
	fn incr_num_successors(&mut self) {
		self.num_successors += 1;
	}

	/// Increment number of times visited (used in topological sort)
	pub fn incr_times_visited(&mut self) {
		self.times_visited += 1;
	}

	/// Decrement number of successors
	fn decr_num_successors(&mut self) {
		self.num_successors -= 1;
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

	pub fn data_mut(&mut self) -> &mut T {
		&mut self.data
	}

	pub fn dedup_after(&mut self) {
		self.req.sort_unstable();
		self.req.dedup();
	}

	pub fn dedup_before(&mut self) {
		self.incl.sort_unstable();
		self.incl.dedup();
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
			self.remove_predecessor_by_index(index.unwrap());
		}
	}

	/// Remove predecessor node, given its index; does nothing if `index`
	/// does not correspond to a predecessor node
	pub fn remove_predecessor_by_index(
		&mut self,
		index: usize,
	) {
		if index < self.predecessors().len() {
			self.predecessors[index].borrow_mut().decr_num_successors();
			self.predecessors.remove(index);
		}
	}

	/// Compute cost of graph with this node as root; ignores cycles;
	/// required for sorting branches;
	pub fn compute_dag_cost(&mut self) -> usize {
		let self_path = { self.path.clone() };
		for it in self.predecessors.iter() {
			// There are still cycles that we need to ignore
			let it_path = { it.borrow().path.clone() };
			let cycle = { it.try_borrow_mut().is_err() };
			if cycle == false {
				self.dag_cost += it.borrow_mut().compute_dag_cost();
			} else {
				println!("{} forms a cycle with{}", self_path, it_path);
			}
		}
		self.dag_cost
	}

	/// Sort predecessors by tree cost so that branches with deadlines
	/// appear first, earlier deadlines appear earlier than later
	/// deadlines, branches without deadlines appear later, and shorter
	/// branches without deadlines appear before/after longer branches
	/// without deadlines, depending on option selected
	pub fn sort_predecessor_branches(
		&mut self,
		reverse: bool,
		compare: NodeComparison<T>,
	) {
		self.predecessors.sort_by(|a, b| compare(reverse, a, b));
	}
}

/// Compare DAG cost between Nodes, used for sorting branches without
/// deadlines
pub fn compare_dag_cost<T>(
	reverse: bool,
	a: &Rc<RefCell<Node<T>>>,
	b: &Rc<RefCell<Node<T>>>,
) -> Ordering {
	if reverse == true {
		a.borrow().dag_cost.cmp(&b.borrow().dag_cost)
	} else {
		b.borrow().dag_cost.cmp(&a.borrow().dag_cost)
	}
}
