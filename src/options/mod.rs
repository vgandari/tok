pub struct Options {
	pub files: Vec<String>,
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
	pub depth: i32,
}

impl Options {
	pub fn new(matches: clap::ArgMatches) -> Self {
		Options {
			files: matches.values_of_lossy("FILES").unwrap(),
			reverse: matches.is_present("reverse"),
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
			depth: matches
				.value_of("depth")
				.unwrap_or("-1")
				.parse::<i32>()
				.unwrap_or(-1),
		}
	}
}
