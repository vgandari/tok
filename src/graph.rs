use crate::node::{add_predecessor_node, add_successor_node, load, Node};
use std::{cell::RefCell, collections::HashMap, collections::HashSet, rc::Rc};

/// Build directed acyclic graph from nodes
pub fn build_dag_from_nodes() {}

/// Given a root node, remove indirect predecessors
pub fn dag_to_tree() {}

/// Load nodes declared in after and before, starting with after
pub fn build_dag_backward<T, U>(
  node: Rc<RefCell<Node<T>>>,
  nodes: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
  pbranch: &mut HashSet<String>,
  sbranch: &mut HashSet<String>,
  read_from_file: fn(&String) -> U,
  create_node: fn(&String, U) -> Rc<RefCell<Node<T>>>,
) {
  // Iterate over predecessor paths
  let after_paths = node.borrow().after.clone();
  for it_dirty in after_paths.iter() {
    let it = &it_dirty.replace("../", "").replace("./", "");
    // Do not add predecessors if they form a cycle
    if pbranch.contains(it) == false {
      // Add file name to list of paths on branch
      pbranch.insert(it.clone());

      // Load node from file
      load(nodes, it, read_from_file, create_node);

      // Recursion
      let predecessor_node = nodes.get(it).unwrap().clone();
      build_dag_forward(
        predecessor_node.clone(),
        nodes,
        pbranch,
        sbranch,
        read_from_file,
        create_node,
      );
      add_predecessor_node(node.clone(), predecessor_node.clone());

      // Exit branch
      pbranch.remove(it);
    }
  }
}

pub fn build_dag_forward<T, U>(
  node: Rc<RefCell<Node<T>>>,
  nodes: &mut HashMap<String, Rc<RefCell<Node<T>>>>,
  pbranch: &mut HashSet<String>,
  sbranch: &mut HashSet<String>,
  read_from_file: fn(&String) -> U,
  create_node: fn(&String, U) -> Rc<RefCell<Node<T>>>,
) {
  // Iterate over successor paths
  let before_paths = node.borrow_mut().before.clone();
  for it_dirty in before_paths.iter() {
    let it = &it_dirty.replace("../", "").replace("./", "");
    // Do not add successors if they form a cycle
    if sbranch.contains(it) == false {
      // Add file name to list of paths on branch
      sbranch.insert(it.clone());

      // Load node from file
      load(nodes, it, read_from_file, create_node);

      // Recursion
      let successor_node = nodes.get(it).unwrap().clone();
      build_dag_forward(
        successor_node.clone(),
        nodes,
        pbranch,
        sbranch,
        read_from_file,
        create_node,
      );
      // if depth > 0 {
      // depth -= 1;
      // }
      let root = nodes.get(&"//".to_string()).unwrap().clone();
      add_successor_node(root, node.clone(), successor_node.clone());

      // Exit branch
      sbranch.remove(it);
    }
  }
  build_dag_backward(
    node.clone(),
    nodes,
    pbranch,
    sbranch,
    read_from_file,
    create_node,
  );
}

pub fn remove_indirect_predecessors<T>(root: Rc<RefCell<Node<T>>>, node: Rc<RefCell<Node<T>>>) {
  for pred in node.borrow().predecessors() {
    if pred != root {
      let valid_borrow = { root.try_borrow_mut().is_ok() };
      if valid_borrow == true {
        root.borrow_mut().remove_predecessor(pred.clone());
      }
    }
  }
}

/// Modified Depth First Search to add predecessors to tree; requires
/// branches to be sorted
pub fn topological_sort<T>(node: Rc<RefCell<Node<T>>>) -> Vec<Rc<RefCell<Node<T>>>> {
  let mut stack = vec![node.clone()];
  let mut sorted_nodes = vec![];
  while stack.is_empty() == false {
    let v = stack.pop().unwrap();
    if v.borrow().is_discovered() == false {
      // This condition prevents nodes from being marked discovered
      // prematurely
      v.borrow_mut().mark_discovered();

      // if v.borrow().has_single_successor() {
      //   v.borrow_mut().mark_discovered();
      // }

      // This is part of the normal DFS
      for w in v.borrow().predecessors().iter() {
        // if w.borrow().has_multiple_successors() {
        //   w.borrow_mut().decr_num_successors();
        // } else {
        stack.push(w.clone());
        // }
      }

      if v.borrow().is_discovered() == true {
        sorted_nodes.push(v.clone());
      }
    }
  }
  sorted_nodes
}
