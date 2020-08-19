use crate::{node::Node, options::Options, topic::Topic};
use std::{
	cell::RefCell,
	fs::File,
	io::{prelude::*, Write},
	path::Path,
	process::Command,
	rc::Rc,
};

// TODO: Show time remaining before deadline
fn print_deadline(
	node: Rc<RefCell<Node<Topic>>>,
	file: &mut File,
) {
	// Show deadline
	if node.borrow().data().deadline.is_some() {
		file.write_all(b"\\noindent").expect("");
		let s: Vec<String> = node
			.borrow()
			.data()
			.deadline
			.clone()
			.unwrap()
			.iter()
			.map(|x| {
				if x < &10 {
					let mut s = 0.to_string();
					s.push_str(&x.to_string());
					s
				} else {
					x.to_string()
				}
			})
			.collect();
		file.write_all(b"\\textbf{Deadline:} ").expect("");
		file.write_all(s.join(&"-"[..]).as_bytes()).expect("");
		file.write_all(b"\n\n").expect("");
	}
}

fn print_start_end_dates(
	node: Rc<RefCell<Node<Topic>>>,
	file: &mut File,
) {
	// Do not indent start/end dates
	if node.borrow().data().start.is_some()
		|| node.borrow().data().complete.is_some()
	{
		file.write_all(b"\\noindent").expect("");
	}

	// Show start date if present
	if node.borrow().data().start.is_some() {
		let s: Vec<String> = node
			.borrow()
			.data()
			.start
			.clone()
			.unwrap()
			.iter()
			.map(|x| {
				if x < &10 {
					let mut s = 0.to_string();
					s.push_str(&x.to_string());
					s
				} else {
					x.to_string()
				}
			})
			.collect();
		file.write_all(b"\\textbf{Begin:} ").expect("");
		file.write_all(s.join(&"-"[..]).as_bytes()).expect("");
		if node.borrow().data().complete.is_some() {
			file.write_all(b", ").expect("");
		}
	}

	// Show completion date if present
	if node.borrow().data().complete.is_some() {
		let s: Vec<String> = node
			.borrow()
			.data()
			.complete
			.clone()
			.unwrap()
			.iter()
			.map(|x| {
				if x < &10 {
					let mut s = 0.to_string();
					s.push_str(&x.to_string());
					s
				} else {
					x.to_string()
				}
			})
			.collect();
		file.write_all(b"\\textbf{End:} ").expect("");
		file.write_all(s.join(&"-"[..]).as_bytes()).expect("");
	}

	// Show actual duration
	// NOTE: Expect duration to be > 0 iff start and comple are defined
	if node.borrow().data().duration > 0 {
		file.write_all(b", \\textbf{Actual Duration:} ").expect("");
		file
			.write_all(node.borrow().data().duration.to_string().as_bytes())
			.expect("");
		file.write_all(b" days").expect("");
	}
	file.write_all(b"\n\n").expect("");
}

pub fn compile_pdf(options: &Options) {
	// set output directory for tex file
	let latex_args =
		["-output-directory=../output", "../output/main.tex"];
	let mut latex_cmd = match options.engine.as_str() {
		"pdflatex" => Command::new("pdflatex"),
		"lualatex" => Command::new("lualatex"),
		"xelatex" => Command::new("xelatex"),
		_ => Command::new("xelatex"),
	};
	latex_cmd.current_dir("../output").args(&latex_args);

	// Run LaTeX
	println!("Compiling PDF ...");
	match options.engine.as_str() {
		"pdflatex" => println!("Running PDFLaTeX (1 of 3) ..."),
		"lualatex" => println!("Running LuaLaTeX (1 of 3) ..."),
		"xelatex" => println!("Running XeLaTeX (1 of 3) ..."),
		_ => println!("Running XeLaTeX (1 of 3) ..."),
	};
	let _ = latex_cmd
		.output()
		.expect("LaTeX command failed to start; it may not be installed");

	// BibTeX
	println!("Running BibTeX ...");
	let _ = Command::new("bibtex")
		.current_dir("../output")
		.arg("main")
		.output()
		.expect("bibtex failed to start");

	// Rerun LaTeX
	match options.engine.as_str() {
		"pdflatex" => println!("Running PDFLaTeX (2 of 3) ..."),
		"lualatex" => println!("Running LuaLaTeX (2 of 3) ..."),
		"xelatex" => println!("Running XeLaTeX (2 of 3) ..."),
		_ => println!("Running XeLaTeX (2 of 3) ..."),
	};
	let _ = latex_cmd
		.output()
		.expect("LaTeX command failed to start; it may not be installed");
	match options.engine.as_str() {
		"pdflatex" => println!("Running PDFLaTeX (3 of 3) ..."),
		"lualatex" => println!("Running LuaLaTeX (3 of 3) ..."),
		"xelatex" => println!("Running XeLaTeX (3 of 3) ..."),
		_ => println!("Running XeLaTeX (3 of 3) ..."),
	};
	let _ = latex_cmd
		.output()
		.expect("LaTeX command failed to start; it may not be installed");
	println!("Finished compiling PDF.");
	println!("Check logfiles for any errors.");

	if options.output.is_empty() == false {
		println!("{}", &options.output);
		let output_path: &str = &options.output[..];
		let cp_args = ["../output/main.pdf", output_path];
		let _ = Command::new("cp")
			.args(&cp_args)
			.output()
			.expect("No PDF to copy to output path");
	}
}

/// Write proofs; meant for a node that contains text for a theorem
/// environment
fn write_proofs(
	options: &Options,
	node: Rc<RefCell<Node<Topic>>>,
	file: &mut File,
) {
	if options.show_proofs == true {
		if node.borrow().data().pfs.is_empty() == false {
			for proof in &node.borrow().data().pfs {
				file.write_all(b"\n\\begin{proof}\n").expect("");
				file.write_all(proof.as_bytes()).expect("");
				file.write_all(b"\\end{proof}\n\n").expect("");
			}
		}
	}
}

/// Write text stored in nodes to tex file
pub fn write_to_tex(
	options: &Options,
	sorted_nodes: &Vec<Rc<RefCell<Node<Topic>>>>,
	mut final_nodes: Vec<String>,
	max_heading_depth: usize,
) {
	// generate tex file
	let path = Path::new("../output/main.tex");

	// Open a file in write-only mode, returns `io::Result<File>`
	let mut file =
		File::create(&path).expect("could not create tex file");

	println!("Writing tex file ...");
	const DEFAULT_PREAMBLE: &'static str =
		include_str!("defaults/default_preamble.tex");
	const DEFAULT_FRONTMATTER: &'static str =
		include_str!("defaults/default_frontmatter.tex");
	const DEFAULT_BACKMATTER: &'static str =
		include_str!("defaults/default_backmatter.tex");

	// choose preamble
	let preamble: String =
		if Path::new("../texinput/preamble.tex").is_file() {
			let mut file = File::open("../texinput/preamble.tex")
				.expect("Cannot read preamble");
			let mut contents = String::new();
			file
				.read_to_string(&mut contents)
				.expect("Cannot read to string");
			println!("Using custom preamble");
			contents
		} else {
			println!("Using default preamble");
			DEFAULT_PREAMBLE.to_string()
		};
	// choose frontmatter
	let frontmatter: String =
		if Path::new("../texinput/frontmatter.tex").is_file() {
			let mut file = File::open("../texinput/frontmatter.tex")
				.expect("Cannot read frontmatter");
			let mut contents = String::new();
			file
				.read_to_string(&mut contents)
				.expect("Cannot read to string");
			println!("Using custom frontmatter");
			contents
		} else {
			println!("Using default frontmatter");
			DEFAULT_FRONTMATTER.to_string()
		};
	// choose backmatter
	let backmatter: String =
		if Path::new("../texinput/backmatter.tex").is_file() {
			let mut file = File::open("../texinput/backmatter.tex")
				.expect("Cannot read backmatter");
			let mut contents = String::new();
			file
				.read_to_string(&mut contents)
				.expect("Cannot read to string");
			println!("Using custom backmatter");
			contents
		} else {
			println!("Using default backmatter");
			DEFAULT_BACKMATTER.to_string()
		};

	// Write preamble to file
	file.write_all(preamble.as_bytes()).expect("");
	file.write_all(b"\n\n").expect("");

	// Write title
	if options.title.is_empty() == false {
		file.write_all(b"\\title{").expect("");
		file.write_all(options.title.as_bytes()).expect("");
		file.write_all(b"}\n").expect("");
	}

	// Write author
	if options.author.is_empty() == false {
		file.write_all(b"\\author{").expect("");
		file.write_all(options.author.as_bytes()).expect("");
		file.write_all(b"}\n").expect("");
	}

	// Write empty date
	if options.date.is_empty() == false {
		file.write_all(b"\\date{").expect("");
		file.write_all(options.date.as_bytes()).expect("");
		file.write_all(b"}\n").expect("");
	}

	// Write title and frontmatter to file
	file.write_all(b"\n").expect("");
	file.write_all(b"\\begin{document}").expect("");
	file.write_all(b"\n\n\\maketitle\n\n").expect("");

	if frontmatter.is_empty() == false {
		file.write_all(b"\\frontmatter\n\n").expect("");
		file.write_all(frontmatter.as_bytes()).expect("");
		file.write_all(b"\n\n\\mainmatter\n\n").expect("");
	}

	// Write content in each node
	let mut write_appendix = false;
	for node in &mut sorted_nodes.iter().rev() {
		let node_path = node.borrow().path.clone();
		if final_nodes.is_empty() == false
			&& final_nodes.contains(&node_path)
		{
			// Remove nodes from final nodes list until exhausted;
			// Do not insert appendix
			write_appendix = false;
			let index =
				final_nodes.iter().position(|x| x == &node_path).unwrap();
			final_nodes.remove(index);
		} else if final_nodes.is_empty() && write_appendix == false {
			// Insert appendix only first time final nodes list is exhausted;
			// Do not insert appendix thereafter;
			// If user suppresses appendix, don't print
			write_appendix = options.write_appendix;
			file
				.write_all(b"\n\\appendix\n\\section{Appendix}\n\n")
				.expect("");
		}

		// Write heading title
		let heading_cmds = match max_heading_depth {
			0 => vec!["", "", "", "", "", ""],
			1 => vec!["", "section", "", "", "", "", ""],
			2 => vec!["", "section", "subsection", "", "", "", ""],
			3 => vec!["", "chapter", "section", "subsection", "", "", ""],
			4 => vec![
				"",
				"chapter",
				"section",
				"subsection",
				"subsubsection",
				"",
				"",
			],
			5 => vec![
				"",
				"part",
				"chapter",
				"section",
				"subsection",
				"subsubsection",
				"",
			],
			// ignore anything deeper than 6 levels
			_ => vec![
				"",
				"book",
				"part",
				"chapter",
				"section",
				"subsection",
				"subsubsection",
			],
		};
		// Write heading title
		let heading_label_pfx = match max_heading_depth {
			0 => vec!["", "", "", "", "", ""],
			1 => vec!["", "sec", "", "", "", "", ""],
			2 => vec!["", "sec", "ssec", "", "", "", ""],
			3 => vec!["", "ch", "sec", "ssec", "", "", ""],
			4 => vec!["", "ch", "sec", "ssec", "sssec", "", ""],
			5 => vec!["", "pt", "ch", "sec", "ssec", "sssec", ""],
			// ignore anything deeper than 6 levels
			_ => vec!["", "bk", "pt", "ch", "sec", "ssec", "sssec"],
		};

		let mut i = node.borrow().data().heading_depth_start;
		for ht in node.borrow().data().heading_titles.clone() {
			if i <= max_heading_depth {
				if ht.is_empty() == false {
					file.write_all(b"\\").expect("");
					file.write_all(heading_cmds[i].as_bytes()).expect("");
					file.write_all(b"{").expect("");
					file.write_all(ht.as_bytes()).expect("");
					file.write_all(b"}\\label{").expect("");
					file.write_all(heading_label_pfx[i].as_bytes()).expect("");
					file.write_all(b":").expect("");
					file.write_all(node.borrow().path.as_bytes()).expect("");
					file.write_all(b"}\n\n").expect("");
				}
			}
			i += 1;
		}

		// Write source YAML file name
		if options.yaml == true {
			file
				.write_all(b"\\noindent\n\\begin{verbatim}\n")
				.expect("");
			file.write_all(node.borrow().path.as_bytes()).expect("");
			file.write_all(b"\n\\end{verbatim}\n\n").expect("");
		}

		// Write label in bold text if env is `plain`
		if node.borrow().data().env.as_str() == "plain" {
			if node.borrow().data().label.is_empty() == false {
				file.write_all(b"\n\\noindent\n\\textbf{").expect("");
				file
					.write_all(node.borrow().data().label.as_bytes())
					.expect("");
				file.write_all(b"}\n\n").expect("");
			}
		}

		// If environment is a task, print status before "pre" text
		match node.borrow().data().env.as_str() {
			// Completed task; useful if completion date is unknown
			"done" => {
				file.write_all(b"\n\\noindent\n\\textbf{").expect("");
				file
					.write_all(node.borrow().data().label.as_bytes())
					.expect("");
				file.write_all(b"}").expect("");

				// Show task status
				file
					.write_all(b"\\marginpar{\\ding{51} \\textbf{DONE}}\n")
					.expect("");
				file.write_all(b"\\reversemarginpar\n\n").expect("");

				// Print deadline, start, and end dates
				print_deadline(node.clone(), &mut file);
				print_start_end_dates(node.clone(), &mut file);
			}

			// Task not started, or with more info
			"task" => {
				// Display label
				file.write_all(b"\n\\noindent\n\\textbf{").expect("");
				file
					.write_all(node.borrow().data().label.as_bytes())
					.expect("");
				file.write_all(b"}").expect("");

				// Show task status
				if node.borrow().data().complete.is_some() {
					file
						.write_all(b"\\marginpar{\\ding{51} \\textbf{DONE}}\n")
						.expect("");
				} else {
					file
						.write_all(b"\\marginpar{$\\square$  \\textbf{TO DO}}\n")
						.expect("");
				}
				file.write_all(b"\\reversemarginpar\n\n").expect("");

				// Display expected duration for task if incomplete
				if node.borrow().data().complete.is_none() {
					if node.borrow().data().expected > 0 {
						file
							.write_all(b"\\noindent\\textbf{Expected Duration:} ")
							.expect("");
						file
							.write_all(
								node.borrow().data().expected.to_string().as_bytes(),
							)
							.expect("");
						file.write_all(b" days\n\n").expect("");
					}
				}

				// Print deadline, start, and end dates
				print_deadline(node.clone(), &mut file);
				print_start_end_dates(node.clone(), &mut file);
			}
			_ => (),
		}

		// Write ELI5 text
		if options.eli5 == true
			&& node.borrow().data().eli5.is_empty() == false
		{
			file
				.write_all(node.borrow().data().eli5.as_bytes())
				.expect("");
			file.write_all(b"\n").expect("");
		}

		// Write pretext
		if options.crib == false
			&& node.borrow().data().pre.is_empty() == false
		{
			file
				.write_all(node.borrow().data().pre.as_bytes())
				.expect("");
			file.write_all(b"\n").expect("");
		}

		// Write main text
		match node.borrow().data().env.as_str() {
			// Task
			"task" => {
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
			}
			// Completed Task
			"done" => {
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
			}
			// Motivation
			"mot" => (),
			// Abstract
			"abs" => {
				if options.crib == false {
					file.write_all(b"\\begin{abstract}\n").expect("");
					file
						.write_all(node.borrow().data().main.as_bytes())
						.expect("");
					file.write_all(b"\\end{abstract}\n\n").expect("");
				}
			}
			// Definition
			"def" => {
				file.write_all(b"\\begin{definition}[").expect("");
				file
					.write_all(node.borrow().data().label.as_bytes())
					.expect("");
				file.write_all(b"]\\label{def:").expect("");
				file.write_all(node.borrow().path.as_bytes()).expect("");
				file.write_all(b"}\n").expect("");
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
				file.write_all(b"\\end{definition}\n\n").expect("");
			}
			// Example
			"eg" => {
				if options.examples == true {
					file.write_all(b"\\begin{example}[").expect("");
					file
						.write_all(node.borrow().data().label.as_bytes())
						.expect("");
					file.write_all(b"]\\label{eg:").expect("");
					file.write_all(node.borrow().path.as_bytes()).expect("");
					file.write_all(b"}\n").expect("");
					file
						.write_all(node.borrow().data().main.as_bytes())
						.expect("");
					file.write_all(b"\\end{example}\n\n").expect("");
				}
			}
			// Lemma
			"lem" => {
				file.write_all(b"\\begin{lemma}[").expect("");
				file
					.write_all(node.borrow().data().label.as_bytes())
					.expect("");
				file.write_all(b"]\\label{lem:").expect("");
				file.write_all(node.borrow().path.as_bytes()).expect("");
				file.write_all(b"}\n").expect("");
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
				file.write_all(b"\\end{lemma}\n\n").expect("");
				write_proofs(&options, node.clone(), &mut file);
			}
			// Theorem
			"thm" => {
				file.write_all(b"\\begin{theorem}[").expect("");
				file
					.write_all(node.borrow().data().label.as_bytes())
					.expect("");
				file.write_all(b"]\\label{thm:").expect("");
				file.write_all(node.borrow().path.as_bytes()).expect("");
				file.write_all(b"}\n").expect("");
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
				file.write_all(b"\\end{theorem}\n\n").expect("");
				write_proofs(&options, node.clone(), &mut file);
			}
			// Corollary
			"cor" => {
				file.write_all(b"\\begin{corollary}[").expect("");
				file
					.write_all(node.borrow().data().label.as_bytes())
					.expect("");
				file.write_all(b"]\\label{cor:").expect("");
				file.write_all(node.borrow().path.as_bytes()).expect("");
				file.write_all(b"}\n").expect("");
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
				file.write_all(b"\\end{corollary}\n\n").expect("");
				write_proofs(&options, node.clone(), &mut file);
			}
			// Rule
			"rule" => {
				file.write_all(b"\\begin{rule}[").expect("");
				file
					.write_all(node.borrow().data().label.as_bytes())
					.expect("");
				file.write_all(b"]\\label{rule:").expect("");
				file.write_all(node.borrow().path.as_bytes()).expect("");
				file.write_all(b"}\n").expect("");
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
				file.write_all(b"\\end{rule}\n\n").expect("");
				write_proofs(&options, node.clone(), &mut file);
			}
			// Fact
			"fact" => {
				file.write_all(b"\\begin{fact}[").expect("");
				file
					.write_all(node.borrow().data().label.as_bytes())
					.expect("");
				file.write_all(b"]\\label{fact:").expect("");
				file.write_all(node.borrow().path.as_bytes()).expect("");
				file.write_all(b"}\n").expect("");
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
				file.write_all(b"\\end{fact}\n\n").expect("");
				write_proofs(&options, node.clone(), &mut file);
			}
			// Remark
			"rem" => {
				file.write_all(b"\\begin{remark}[").expect("");
				file
					.write_all(node.borrow().data().label.as_bytes())
					.expect("");
				file.write_all(b"]").expect("");
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
				file.write_all(b"\\end{remark}\n\n").expect("");
			}
			// Algorithm
			"alg" => (),
			// Code listing from text
			"lst" => {
				if options.crib == false {
					file
						.write_all(node.borrow().data().main.as_bytes())
						.expect("");
					file.write_all(b"\\begin{lstlisting}").expect("");
					if node.borrow().data().lang.is_empty() == false {
						file.write_all(b"[language=").expect("");
						file
							.write_all(node.borrow().data().lang.as_bytes())
							.expect("");
						file.write_all(b"]").expect("");
					}
					file.write_all(b"\n").expect("");
					file
						.write_all(node.borrow().data().listtext.as_bytes())
						.expect("");
					file.write_all(b"\n").expect("");
					file.write_all(b"\\end{lstlisting}\n\n").expect("");
				}
			}

			// Code listing from file
			"lstfile" => {
				if options.crib == false {
					file
						.write_all(node.borrow().data().pre.as_bytes())
						.expect("");
					file.write_all(b"\\lstinputlisting").expect("");
					if node.borrow().data().lang.is_empty() == false {
						file.write_all(b"[language=").expect("");
						file
							.write_all(node.borrow().data().lang.as_bytes())
							.expect("");

						if node.borrow().data().lines.is_empty() == false {
							file.write_all(b", firstline=").expect("");
							file
								.write_all(
									node.borrow().data().lines[0].to_string().as_bytes(),
								)
								.expect("");
							file.write_all(b", lastline=").expect("");
							file
								.write_all(
									node.borrow().data().lines[1].to_string().as_bytes(),
								)
								.expect("");
						}
						file.write_all(b"]").expect("");
					}
					file.write_all(b"{").expect("");
					file.write_all(b"\n").expect("");
					file
						.write_all(node.borrow().data().listtext.as_bytes())
						.expect("");
					file.write_all(b"}\n").expect("");
					file
						.write_all(node.borrow().data().main.as_bytes())
						.expect("");
				}
			}

			// Plain text
			_ => {
				// file.write_all(b"\n").expect("");
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
			}
		}

		// Print alternate names
		if node.borrow().data().aka.len() > 0 {
			file
			.write_all(
				b"\n\n\\noindent\\textbf{Also known as:}\\newline\n\\noindent\n",
			)
			.expect("");
			file.write_all(b"\\begin{itemize}\n").expect("");
			for name in node.borrow().data().aka.clone() {
				file.write_all(b"\\item ").expect("");
				file.write_all(name.as_bytes()).expect("");
				file.write_all(b"\n").expect("");
			}
			file.write_all(b"\\end{itemize}\n").expect("");
		}

		// Link to Wikipedia
		if (options.crib == false)
			& (options.show_wiki == true)
			& (node.borrow().data().env != "x")
			& (node.borrow().data().nowiki == false)
		{
			file.write_all(b"\n\n\\noindent\n").expect("");
			file.write_all(b"\\href{").expect("");
			if node.borrow().data().wiki.is_empty() == true {
				let wiki_search_url: String =
					"https://en.wikipedia.org/w/index.php?search=".to_string();
				let wiki_search_term: String =
					node.borrow().data().label.clone();
				file
					.write_all((wiki_search_url + &wiki_search_term).as_bytes())
					.expect("");
			} else if options.crib == false {
				file
					.write_all(node.borrow().data().wiki.as_bytes())
					.expect("");
			}

			// Hyperlink label
			file.write_all(b"}{").expect("");
			if node.borrow().data().wiki.is_empty() == true {
				// If author does not provide direct link, do not suggest that
				// the link merely searches for the Wikipedia page
				file.write_all(b"Search for ").expect("");
			}
			file.write_all(b"``").expect("");
			file
				.write_all(node.borrow().data().label.as_bytes())
				.expect("");
			file.write_all(b"'' on Wikipedia}\n\n").expect("");
		}

		// Write additional discussion/commentary after main text
		if (options.crib == false)
			& (node.borrow().data().post.is_empty() == false)
		{
			file
				.write_all(node.borrow().data().post.as_bytes())
				.expect("");
			file.write_all(b"\n").expect("");
		}

		// Links to URLs
		if options.show_urls == true {
			for url in node.borrow().data().urls.iter() {
				file.write_all(b"\\noindent\n").expect("");
				file.write_all(b"\\href{").expect("");
				file.write_all(url.1.as_bytes()).expect("");
				file.write_all(b"}{").expect("");
				file.write_all(url.0.as_bytes()).expect("");
				file.write_all(b"}\n\n").expect("");
			}
		}

		// Questions for author to answer in a future draft
		if (options.crib == false) & (options.show_q == true) {
			if node.borrow().data().q.len() > 0 {
				file
					.write_all(b"\\begin{itemize}\n\\color{red}\n")
					.expect("");
			}
			for it in node.borrow().data().q.clone() {
				file.write_all(b"\\item ").expect("");
				file.write_all(it.as_bytes()).expect("");
				file.write_all(b"\n").expect("");
			}
			if node.borrow().data().q.len() > 0 {
				file.write_all(b"\\end{itemize}\n").expect("");
			}
		}
		file.write_all(b"\n\n").expect("");
	}

	// Write backmatter
	if backmatter.is_empty() == false {
		file.write_all(b"\n\n\\backmatter\n\n").expect("");
		file.write_all(backmatter.as_bytes()).expect("");
	}
	file.write_all(b"\n\n\\end{document}").expect("");
}

/// Generate BibTeX file from sources
pub fn write_bib(sorted_nodes: &Vec<Rc<RefCell<Node<Topic>>>>) {
	let path = Path::new("../output/main.bib");
	let mut file =
		File::create(&path).expect("could not create .bib file");
	let mut references: Vec<String> = vec![];
	for n in sorted_nodes {
		for src in n.borrow().data().src.clone() {
			references.push(src.clone());
			references.sort_unstable();
			references.dedup();
		}
	}
	for r in references {
		file.write_all(r.as_bytes()).expect("");
		file.write_all(b"\n").expect("");
	}
	// This is to ensure that the file exists and LaTeX doesn't fail to
	// compile
	file.write_all(b"\n").expect("");
}
