use crate::node::{compare_dag_cost, Node};
use crate::yaml::DeserializedMap;
use chrono::offset::{TimeZone, Utc};
use std::{cell::RefCell, cmp::Ordering, collections::HashMap, rc::Rc};
use titlecase::titlecase;

pub struct Topic {
	/// Name as it should appear in textbook
	pub label: String,
	/// LaTeX environment
	pub env: String,
	/// "Explain Like I'm Five" explanation; separate from all other text
	pub eli5: String,
	/// Text to add to LaTeX file before main content;
	/// remains outside any environment;
	/// useful for providing brief introduction to the topic
	/// in a given YAML file
	pub pre: String,
	/// Text to add to LaTeX file inside environment declared in `env`
	/// key, if any
	pub main: String,
	/// Text to add to LaTeX file after main content;
	/// remains outside any environment;
	/// useful for providing brief introduction to the topic
	/// in a given YAML file
	pub post: String,
	/// Text to include in a listing (may be a file)
	pub listtext: String,
	/// Specify language for syntax highlighting in listings
	pub lang: String,
	/// Other names used to refer to this topic
	pub aka: Vec<String>,
	/// Lines to include from a listing from a file
	pub lines: Vec<usize>,
	/// List of proofs to place immediately after `main` text (intended
	/// for `env=thm` only);
	/// input as sequence of multiline strings in YAML file
	pub pfs: Vec<String>,
	// example of other items that will not be included as successors
	// pub example_of: Vec<String>,
	// examples of this item to include
	// pub example_paths: Vec<String>,
	/// Link to Wikipedia page; if empty, will result in a link to a
	/// Wikipedia search query for this node's label; if not empty,
	/// author-provided link will be used instead
	pub wiki: String,
	/// Whether to attempt to search for Wikipedia page if `wiki` is
	/// empty; `nowiki==false` by default; i.e. will search for node
	/// label by default
	pub nowiki: bool,
	/// Other author-provided links (e.g. Mathworld, Stack Exchange,
	/// academic websites)
	pub urls: HashMap<String, String>,
	/// Questions for author to answer in subsequent drafts
	pub q: Vec<String>,
	/// Start date for a task
	pub start: Option<Vec<usize>>,
	/// Completion date for a task
	pub complete: Option<Vec<usize>>,
	/// Task deadline
	pub deadline: Option<Vec<usize>>,
	/// Expected duration of a task (in days)
	pub expected: usize,
	/// Actual duration of a task (in days)
	pub duration: usize,
	// we allow different types of predecessors
	// so that we can talk about relationships between items
	pub gen: Vec<String>,
	pub case: Vec<String>,
	/// List of references that will be added to .bib file for this node
	pub src: Vec<String>,
	pub heading_depth: usize,
	pub heading_depth_start: usize,
	/// Heading title if this topic forms the start of a chapter, section,
	/// subsection, etc.
	pub heading_titles: Vec<String>,
}

impl Topic {
	pub fn new() -> Topic {
		Topic {
			label: String::from(""),
			env: String::from(""),
			eli5: String::from(""),
			pre: String::from(""),
			main: String::from(""),
			listtext: String::from(""),
			lang: String::from(""),
			aka: vec![],
			lines: vec![],
			post: String::from(""),
			pfs: vec![],
			// example_of: vec![],
			// example_paths: vec![],
			wiki: String::from(""),
			nowiki: false,
			urls: HashMap::new(),
			q: vec![],
			start: None,
			complete: None,
			deadline: None,
			duration: 0,
			expected: 0,
			gen: vec![],
			case: vec![],
			src: vec![],
			heading_depth: 0,
			heading_depth_start: 0,
			heading_titles: vec![],
		}
	}
}

/// Create a Topic and fill data members based on key/value pairs
pub fn create_topic(
	filename: &String,
	yaml_content: DeserializedMap,
) -> Rc<RefCell<Node<Topic>>> {
	// store content in node
	let node = Node::new(&filename, Topic::new());
	let mut data = Topic::new();

	// Extract environment from filename
	let first_underscore = filename.find('_').unwrap_or(0);
	data.env = if first_underscore > 0 {
		filename[0..first_underscore].to_string()
	} else {
		"".to_string()
	};

	// Extract label from filename
	data.label = {
		// Exclude environment
		let mut label: String = if first_underscore > 0 {
			filename[first_underscore + 1..].to_string()
		} else {
			filename.to_string()
		};
		// Remove file extension
		let file_extension_start = label.rfind('.').unwrap_or(0);
		label = label[0..file_extension_start].to_string();
		// Replace underscores with spaces, change to titlecase
		titlecase(&label.replace("_", " ")[..])
	};

	for (k, v) in yaml_content.pairs {
		match k.as_ref() {
			"req" => {
				node.borrow_mut().req = serde_yaml::from_value(v).expect("")
			}
			"incl" => {
				node.borrow_mut().incl = serde_yaml::from_value(v).expect("")
			}
			"label" => {
				// If user supplied label different from what is in filename,
				// overwrite
				let label_in_yaml_file: String =
					serde_yaml::from_value(v.clone()).expect("");
				if label_in_yaml_file.is_empty() == false {
					data.label = serde_yaml::from_value(v).expect("");
				}
			}
			"aka" => data.aka = serde_yaml::from_value(v).expect(""),
			"lang" => data.lang = serde_yaml::from_value(v).expect(""),
			"eli5" => data.eli5 = serde_yaml::from_value(v).expect(""),
			"pre" => data.pre = serde_yaml::from_value(v).expect(""),
			"main" => data.main = serde_yaml::from_value(v).expect(""),
			"post" => data.post = serde_yaml::from_value(v).expect(""),
			"lsttext" => data.listtext = serde_yaml::from_value(v).expect(""),
			"wiki" => data.wiki = serde_yaml::from_value(v).expect(""),
			"nowiki" => data.nowiki = serde_yaml::from_value(v).expect(""),
			"urls" => data.urls = serde_yaml::from_value(v).expect(""),
			"q" => data.q = serde_yaml::from_value(v).expect(""),
			"pfs" => data.pfs = serde_yaml::from_value(v).expect(""),
			"lines" => data.lines = serde_yaml::from_value(v).expect(""),
			"start" => data.start = serde_yaml::from_value(v).expect(""),
			"expected" => {
				data.expected = serde_yaml::from_value(v).expect("")
			}
			"complete" => {
				data.complete = serde_yaml::from_value(v).expect("")
			}
			"deadline" => {
				data.deadline = serde_yaml::from_value(v).expect("")
			}
			"gen" => data.gen = serde_yaml::from_value(v).expect(""),
			"case" => data.case = serde_yaml::from_value(v).expect(""),
			"src" => data.src = serde_yaml::from_value(v).expect(""),
			_ => (),
		}
	}

	// Update node cost
	if data.env == "task" {
		if data.start.is_some() && data.complete.is_some() {
			// Start and completion dates known; compute duration
			let s = data.start.clone().unwrap();
			let c = data.complete.clone().unwrap();
			let a = Utc.ymd(s[0] as i32, s[1] as u32, s[2] as u32);
			let b = Utc.ymd(c[0] as i32, c[1] as u32, c[2] as u32);
			if b < a {
				panic!("Completion date must not be before start date.")
			} else {
				data.duration = (b - a).num_days() as usize;
			}
			node.borrow_mut().cost = 1 + data.duration;
		} else {
			// Start date or completion date missing; only know expected duration
			node.borrow_mut().cost = 1 + data.expected;
		}
	} else {
		// Not a task; use amount of text as a heuristic for computing cost
		node.borrow_mut().cost = 1
			+ data.main.len() as usize
			+ data.pre.len() as usize
			+ data.post.len() as usize;
	}

	let dag_cost = node.borrow().cost;
	{
		node.borrow_mut().set_dag_cost(dag_cost);
	}
	{
		node.borrow_mut().set_data(data);
	}
	{
		node.borrow_mut().dedup_after();
	}
	{
		node.borrow_mut().dedup_before();
	}
	node.clone()
}

// FIXME: Move trees/DAGs where ROOT node has deadline ahead of
// trees/DAGs where root node has no deadline
pub fn compute_ordering(
	reverse: bool,
	a: &Rc<RefCell<Node<Topic>>>,
	b: &Rc<RefCell<Node<Topic>>>,
) -> Ordering {
	if a.borrow().data().deadline.is_none()
		&& b.borrow().data().deadline.is_none()
	{
		// Nodes do not have deadlines, compute ordering based on dag_cost
		compare_dag_cost(reverse, a, b)
	} else if a.borrow().data().deadline.is_some()
		&& b.borrow().data().deadline.is_none()
	{
		Ordering::Less
	} else if a.borrow().data().deadline.is_none()
		&& b.borrow().data().deadline.is_some()
	{
		Ordering::Greater
	} else {
		// both nodes have deadlines
		// compare deadlines, starting with years
		let adl = a.borrow().data().deadline.clone().unwrap();
		let bdl = b.borrow().data().deadline.clone().unwrap();

		// compare years
		if adl[0] > bdl[0] {
			Ordering::Greater
		} else if adl[0] < bdl[0] {
			Ordering::Less
		} else {
			// same year, compare months
			if adl[1] > bdl[1] {
				Ordering::Greater
			} else if adl[1] < bdl[1] {
				Ordering::Less
			} else {
				// same month, compare dates
				if adl[2] > bdl[2] {
					Ordering::Greater
				} else if adl[2] < bdl[2] {
					Ordering::Less
				} else {
					Ordering::Equal
				}
			}
		}
	}
}
