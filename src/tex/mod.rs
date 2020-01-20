use crate::{node::Node, options::Options, yaml::YamlData};
use std::{
	cell::RefCell,
	fs::File,
	io::{prelude::*, Write},
	path::Path,
	process::Command,
	rc::Rc,
};

pub fn compile_pdf() {
	let mut latex_cmd = Command::new("xelatex");
	let mut bibtex_cmd = Command::new("bibtex");
	latex_cmd.arg("-output-directory=output");
	latex_cmd.arg("output/main.tex");
	// latex_cmd.arg("output/main.tex");
	bibtex_cmd.arg("output/main.aux");
	println!("Compiling PDF ...");
	println!("Running XeLaTeX (1 of 3) ...");
	let _output1 = latex_cmd.output().expect("error");
	println!("Running BibTeX ...");
	let _output3 = bibtex_cmd.output().expect("error");
	println!("Running XeLaTeX (2 of 3) ...");
	let _output4 = latex_cmd.output().expect("error");
	println!("Running XeLaTeX (3 of 3) ...");
	let _output5 = latex_cmd.output().expect("error");
	println!("Finished compiling PDF.");
	println!("Check logfiles for any errors.");
}

/// Write proofs; meant for a node that contains text for a theorem
/// environment
fn write_proofs(
	options: &Options,
	node: Rc<RefCell<Node<YamlData>>>,
	file: &mut File,
) {
	if options.show_proofs == true {
		if node.borrow().data().pfs.is_empty() == false {
			for proof in &node.borrow().data().pfs {
				file.write_all(b"\n\\begin{proof}\n").expect("");
				file.write_all(proof.as_bytes()).expect("");
				file.write_all(b"\\end{proof}\n").expect("");
			}
		}
	}
}

/// Write text stored in nodes to tex file
pub fn write_to_tex(
	options: Options,
	sorted_nodes: Vec<Rc<RefCell<Node<YamlData>>>>,
) {
	// generate tex file
	let mut mkdir_cmd = Command::new("mkdir");
	mkdir_cmd.arg("output");
	let _output = mkdir_cmd.output().expect("error");
	let path = Path::new("output/main.tex");

	// Open a file in write-only mode, returns `io::Result<File>`
	let mut file = File::create(&path).expect("could not create file");

	println!("Writing tex file ...");
	const DEFAULT_PREAMBLE: &'static str =
		include_str!("defaults/default_preamble.tex");
	const DEFAULT_FRONTMATTER: &'static str =
		include_str!("defaults/default_frontmatter.tex");
	const DEFAULT_BACKMATTER: &'static str =
		include_str!("defaults/default_backmatter.tex");

	// choose preamble
	let preamble: String = if Path::new("texinput/preamble.tex").is_file()
	{
		let mut file =
			File::open("texinput/preamble.tex").expect("Can't open file!");
		let mut contents = String::new();
		file
			.read_to_string(&mut contents)
			.expect("Cannot read data");
		contents
	} else {
		println!("Using default preamble");
		DEFAULT_PREAMBLE.to_string()
	};
	// choose frontmatter
	let frontmatter: String =
		if Path::new("texinput/frontmatter.tex").is_file() {
			let mut file = File::open("texinput/frontmatter.tex")
				.expect("Can't open file!");
			let mut contents = String::new();
			file
				.read_to_string(&mut contents)
				.expect("Cannot read data");
			contents
		} else {
			println!("Using default frontmatter");
			DEFAULT_FRONTMATTER.to_string()
		};
	// choose backmatter
	let backmatter: String =
		if Path::new("texinput/backmatter.tex").is_file() {
			let mut file = File::open("texinput/backmatter.tex")
				.expect("Can't open file!");
			let mut contents = String::new();
			file
				.read_to_string(&mut contents)
				.expect("Cannot read data");
			contents
		} else {
			println!("Using default backmatter");
			DEFAULT_BACKMATTER.to_string()
		};

	// Write preamble to file, begin document, and write frontmatter
	file.write_all(b"\n\n").expect("");
	file.write_all(preamble.as_bytes()).expect("");
	file.write_all(b"\n\n").expect("");
	file.write_all(b"\\begin{document}").expect("");
	file.write_all(b"\n\n").expect("");
	file.write_all(frontmatter.as_bytes()).expect("");
	file.write_all(b"\n\n").expect("");

	for node in &mut sorted_nodes.iter().rev() {
		// Write comment containing source YAML file name
		file.write_all(b"% ").expect("");
		file.write_all(node.borrow().path.as_bytes()).expect("");
		file.write_all(b"\n").expect("");
		// Write label if env is unspecified
		if node.borrow().data().env.is_empty() == true {
			file.write_all(b"\\noindent\n{\\bfseries ").expect("");
			file
				.write_all(node.borrow().data().label.as_bytes())
				.expect("");
			file.write_all(b"}\n\n").expect("");
		}

		// Write pretext
		if (options.crib == false)
			& (node.borrow().data().pre.is_empty() == false)
		{
			file
				.write_all(node.borrow().data().pre.as_bytes())
				.expect("");
			file.write_all(b"\n").expect("");
		}

		// Write main text
		match node.borrow().data().env.as_str() {
			// Task
			"unchecked" => {
				file
					.write_all(b"\\noindent\n\\begin{verbatim}\n")
					.expect("");
				file.write_all(node.borrow().path.as_bytes()).expect("");
				file.write_all(b"\n\\end{verbatim}\n").expect("");
				file.write_all(b"\\noindent\n\\begin{Form}\n").expect("");
				file.write_all(b"\\CheckBox[]{}\n").expect("");
				file
					.write_all(node.borrow().data().label.as_bytes())
					.expect("");
				file.write_all(b"\n\\end{Form}\n\n").expect("");
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
			}
			// Completed Task
			"checked" => {
				file
					.write_all(b"\\noindent\n\\begin{verbatim}\n")
					.expect("");
				file.write_all(node.borrow().path.as_bytes()).expect("");
				file.write_all(b"\n\\end{verbatim}\n").expect("");
				file.write_all(b"\\noindent\n\\begin{Form}\n").expect("");
				file.write_all(b"\\CheckBox[checked]{}\n").expect("");
				file
					.write_all(node.borrow().data().label.as_bytes())
					.expect("");
				file.write_all(b"\n\\end{Form}\n").expect("");
				file.write_all(b"\n\\end{Form}\n\n").expect("");
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
				if node.borrow().data().duration > 1 {
					file.write_all(b"\n{\\bfseries Task Duration: ").expect("");
					file
						.write_all(
							node.borrow().data().duration.to_string().as_bytes(),
						)
						.expect("");
					file.write_all(b" days}").expect("");
					file.write_all(b"\n").expect("");
				} else if node.borrow().data().duration > 0 {
					file.write_all(b"\n{\\bfseries Task Duration: ").expect("");
					file
						.write_all(
							node.borrow().data().duration.to_string().as_bytes(),
						)
						.expect("");
					file.write_all(b" day}").expect("");
					file.write_all(b"\n").expect("");
				} else {
					file.write_all(b"\n").expect("");
				}
			}
			// Motivation
			"mot" => (),
			// Abstract
			"abstract" => {
				if options.crib == false {
					file.write_all(b"\\begin{abstract}\n").expect("");
					file
						.write_all(node.borrow().data().main.as_bytes())
						.expect("");
					file.write_all(b"\\end{abstract}\n").expect("");
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
				file.write_all(b"\\end{definition}\n").expect("");
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
					file.write_all(b"\\end{example}\n").expect("");
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
				file.write_all(b"\\end{lemma}\n").expect("");
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
				file.write_all(b"\\end{theorem}\n").expect("");
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
				file.write_all(b"\\end{corollary}\n").expect("");
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
				file.write_all(b"\\end{remark}").expect("");
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
					file.write_all(b"\\end{lstlisting}\n").expect("");
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
				file.write_all(b"\n").expect("");
				file
					.write_all(node.borrow().data().main.as_bytes())
					.expect("");
			}
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

		// Question for author to answer in a future draft
		if (options.crib == false) & (options.show_q == true) {
			if node.borrow().data().q.len() > 0 {
				file
					.write_all(b"\\begin{itemize}\n\\color{red}\n")
					.expect("");
			}
			for it in node.borrow().data().q.clone() {
				// file.write_all(b"\\item {\\red ").expect("");
				file.write_all(b"\\item ").expect("");
				file.write_all(it.as_bytes()).expect("");
				// file.write_all(b"}\n").expect("");
			}
			if node.borrow().data().q.len() > 0 {
				file.write_all(b"\\end{itemize}").expect("");
			}
		}

		// Link to Wikipedia
		if (options.crib == false)
			& (options.show_wiki == true)
			& (node.borrow().data().nowiki == false)
		{
			file.write_all(b"\\noindent\n").expect("");
			file.write_all(b"\\href{").expect("");
			if node.borrow().data().wiki.is_empty() == true {
				let mut wiki_search_url: String =
					"https://en.wikipedia.org/w/index.php?search=".to_string();
				let wiki_search_term = node.borrow().data().label.clone();
				wiki_search_url.push_str(wiki_search_term.as_str());

				file.write_all(wiki_search_url.as_bytes()).expect("");
			} else {
				file
					.write_all(node.borrow().data().wiki.as_bytes())
					.expect("");
			}
			file.write_all(b"}{").expect("");
			file.write_all(b"Wikipedia}\n\n").expect("");
		}
	}

	// Write backmatter
	file.write_all(backmatter.as_bytes()).expect("");
	file.write_all(b"\\end{document}").expect("");
}
