use std::collections::HashMap;

pub struct Task {
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
  /// List of references that will be added to .bib file for this node
  pub src: Vec<String>,
}

impl Task {
  pub fn new() -> Task {
    Task {
      label: String::from(""),
      env: String::from(""),
      eli5: String::from(""),
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
    }
  }
}
