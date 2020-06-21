pub struct Options {
	pub yaml: bool,
	pub reverse: bool,
	pub show_wiki: bool,
	pub show_urls: bool,
	pub show_q: bool,
	pub show_proofs: bool,
	pub crib: bool,
	pub examples: bool,
	pub exercises: bool,
	pub draft: bool,
	pub generalizations_before: bool,
	pub write_appendix: bool,
	pub eli5: bool,
	pub make_pdf: bool,
	pub generate_headings: bool,
	pub depth: i32,
	pub title: String,
	pub author: String,
	pub output: String,
	pub files: Vec<String>,
}

impl Options {
	pub fn new(matches: clap::ArgMatches) -> Self {
		Options {
			reverse: matches.is_present("reverse"),
			yaml: matches.is_present("yaml"),
			show_wiki: matches.is_present("wiki"),
			show_urls: matches.is_present("url"),
			show_q: matches.is_present("questions"),
			show_proofs: !matches.is_present("proofs"),
			crib: matches.is_present("crib"),
			examples: matches.is_present("examples"),
			exercises: matches.is_present("exercises"),
			draft: matches.is_present("draft"),
			generalizations_before: matches
				.is_present("generalizations_before"),
			write_appendix: !matches.is_present("write_appendix"),
			eli5: matches.is_present("eli5"),
			make_pdf: !matches.is_present("make_pdf"),
			generate_headings: matches.is_present("generate_headings"),
			depth: matches
				.value_of("depth")
				.unwrap_or("-1")
				.parse::<i32>()
				.unwrap_or(-1),
			title: matches.value_of("title").unwrap_or("").to_string(),
			author: matches.value_of("author").unwrap_or("").to_string(),
			output: matches.value_of("output").unwrap_or("").to_string(),
			files: matches.values_of_lossy("FILES").unwrap(),
		}
	}
}
