use chrono::offset::{TimeZone, Utc};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::node::Node;
use serde_yaml::Value;
extern crate inflector;
use inflector::Inflector;
pub fn read_from_yaml(contents: &String) -> YamlNode {
	serde_yaml::from_str(contents).unwrap()
}

/// Update data members based on YAML key/value pairs
pub fn update_fields(
	filename: &String,
	yaml_content: YamlNode,
) -> Rc<RefCell<Node<YamlData>>> {
	// store content in node
	let node = Node::new(filename, YamlData::new());
	let mut data = YamlData::new();

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
		let file_extension_start = label.find('.').unwrap_or(0);
		label = label[0..file_extension_start].to_string();
		// Replace underscores with spaces
		label = str::replace(label.as_str(), "_", " ");
		// TODO: Change to title case
		let title_case_label = label.to_title_case();
		title_case_label
	};

	for (k, v) in yaml_content.pairs {
		match k.as_ref() {
			"after" => {
				node.borrow_mut().after = serde_yaml::from_value(v).expect("")
			}
			"before" => {
				node.borrow_mut().before = serde_yaml::from_value(v).expect("")
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
			// "env" => data.env = serde_yaml::from_value(v).expect(""),
			"lang" => data.lang = serde_yaml::from_value(v).expect(""),
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
			"alt" => data.alt = serde_yaml::from_value(v).expect(""),
			"ys" => data.ys = serde_yaml::from_value(v).expect(""),
			"ye" => data.ye = serde_yaml::from_value(v).expect(""),
			"ms" => data.ms = serde_yaml::from_value(v).expect(""),
			"me" => data.me = serde_yaml::from_value(v).expect(""),
			"ds" => data.ds = serde_yaml::from_value(v).expect(""),
			"de" => data.de = serde_yaml::from_value(v).expect(""),
			"gen" => data.gen = serde_yaml::from_value(v).expect(""),
			"case" => data.case = serde_yaml::from_value(v).expect(""),
			"src" => data.src = serde_yaml::from_value(v).expect(""),
			_ => (),
		}
	}
	// Update node cost
	if (data.env == "unchecked") | (data.env == "checked") {
		// node.borrow_mut().cost = 1;
		if (data.ys > 0) | (data.ye > 0) {
			let a = Utc.ymd(data.ys, data.ms, data.ds);
			let b = Utc.ymd(data.ye, data.me, data.de + 1);
			if b > a {
				data.duration = (b - a).num_days();
			}
		}
	} else {
		node.borrow_mut().cost = 1
			+ data.main.len() as u64
			+ data.pre.len() as u64
			+ data.post.len() as u64;
	}
	let tree_cost = node.borrow().cost;
	node.borrow_mut().set_tree_cost(tree_cost);
	node.borrow_mut().set_data(data);
	node.borrow_mut().dedup_after();
	node.borrow_mut().dedup_before();
	node.clone()
}

/// struct containing HashMap of key value pairs in YAML file
#[derive(Serialize, Deserialize)]
pub struct YamlNode {
	#[serde(flatten)]
	pairs: HashMap<String, Value>,
}

pub struct YamlData {
	/// Name as it should appear in textbook
	pub label: String,
	/// LaTeX environment (if defined)
	pub env: String,
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
	/// Lines to include from a listing from a file
	pub lines: Vec<u64>,
	/// List of proofs to place immediately after `main` text (intended
	/// for `env=thm` only);
	/// input as sequence of multiline strings in YAML file
	pub pfs: Vec<String>,
	/// Alternate labels
	pub alt: Vec<String>,
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
	// bibliography items
	// sources: BibTeXItem,
	/// Questions for author to answer in subsequent drafts
	pub q: Vec<String>,
	/// Year start (for task types)
	pub ys: i32,
	/// Month start (for task types)
	pub ms: u32,
	/// Date start (for task types)
	pub ds: u32,
	/// Year end (for task types)
	pub ye: i32,
	/// Month end (for task types)
	pub me: u32,
	/// Date end (for task types)
	pub de: u32,
	/// Duration (for task types)
	pub duration: i64,
	// we allow different types of predecessors
	// so that we can talk about relationships between items
	pub gen: Vec<String>,
	pub case: Vec<String>,
	/// List of references that will be added to .bib file for this node
	pub src: Vec<String>,
	/// Headings and subheadings that follow this node in the document
	heading: String,
	/// Name to give section that culminates in this node; will default
	/// to label if empty
	sec: String,
}

impl YamlData {
	pub fn new() -> YamlData {
		YamlData {
			label: String::from(""),
			env: String::from(""),
			pre: String::from(""),
			main: String::from(""),
			listtext: String::from(""),
			lang: String::from(""),
			lines: vec![],
			post: String::from(""),
			pfs: vec![],
			alt: vec![],
			// example_of: vec![],
			// example_paths: vec![],
			wiki: String::from(""),
			nowiki: false,
			urls: HashMap::new(),
			q: vec![],
			ys: 0,
			ms: 1,
			ds: 1,
			ye: 0,
			me: 1,
			de: 1,
			duration: 0,
			gen: vec![],
			case: vec![],
			src: vec![],
			sec: String::from(""),
			heading: String::from(""),
		}
	}

	pub fn heading(&self) -> String {
		self.heading.clone()
	}

	pub fn append_heading(
		&mut self,
		s: &String,
	) {
		self.heading += s;
	}
}
