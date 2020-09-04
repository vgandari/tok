use titlecase::titlecase;

pub struct Options {
	pub verbose: bool,
	pub yaml: bool,
	pub reverse: bool,
	pub show_wiki: bool,
	pub show_urls: bool,
	pub show_q: bool,
	pub show_proofs: bool,
	pub crib: bool,
	pub examples: bool,
	pub exercises: bool,
	pub generalizations_before: bool,
	pub write_appendix: bool,
	pub eli5: bool,
	pub make_pdf: bool,
	pub make_tex: bool,
	pub generate_headings: bool,
	pub extra_headings: bool,
	pub depth: i32,
	pub title: String,
	pub author: String,
	pub date: String,
	pub output: String,
	pub engine: String,
	pub files: Vec<String>,
}

impl Options {
	pub fn new(matches: clap::ArgMatches) -> Self {
		Options {
			verbose: matches.is_present("verbose"),
			reverse: matches.is_present("reverse"),
			yaml: matches.is_present("yaml"),
			show_wiki: matches.is_present("wiki"),
			show_urls: matches.is_present("url"),
			show_q: matches.is_present("questions"),
			show_proofs: !matches.is_present("proofs"),
			crib: matches.is_present("crib"),
			examples: matches.is_present("examples"),
			exercises: matches.is_present("exercises"),
			generalizations_before: matches
				.is_present("generalizations_before"),
			write_appendix: !matches.is_present("write_appendix"),
			eli5: matches.is_present("eli5"),
			make_pdf: !matches.is_present("make_pdf"),
			make_tex: !matches.is_present("make_tex"),
			generate_headings: matches.is_present("generate_headings"),
			extra_headings: matches.is_present("extra_headings"),
			depth: matches
				.value_of("depth")
				.unwrap_or("-1")
				.parse::<i32>()
				.unwrap_or(-1),
			// Replace title with title from command line, or if none given
			// and only single file is passed as input, replace title with
			// single file's label
			title: if matches.values_of_lossy("FILES").unwrap().len() == 1
				&& matches
					.value_of("title")
					.unwrap_or("")
					.to_string()
					.is_empty()
					== true
			{
				// Get filename
				let mut filename = matches.values_of_lossy("FILES").unwrap()[0]
					.replace("./", "");

				// Remove file extension
				let file_extension_start = filename.rfind('.').unwrap_or(0);
				filename = filename[0..file_extension_start].to_string();

				// Extract label from filename, exclude environment
				let first_underscore = filename.find('_').unwrap_or(0);
				let label: String = if first_underscore > 0 {
					filename[first_underscore + 1..].to_string()
				} else {
					filename.to_string()
				};

				// Replace underscores with spaces, change to titlecase
				titlecase(&label.replace("_", " ")[..])
			} else {
				println!("{}", matches.values_of_lossy("FILES").unwrap().len());
				matches.value_of("title").unwrap_or("").to_string()
			},
			author: matches.value_of("author").unwrap_or("").to_string(),
			date: matches.value_of("date").unwrap_or("").to_string(),
			engine: matches.value_of("engine").unwrap_or("").to_string(),
			output: matches.value_of("output").unwrap_or("").to_string(),
			files: matches.values_of_lossy("FILES").unwrap(),
		}
	}
}
