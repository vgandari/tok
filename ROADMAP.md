# ROADMAP

## To Do

### Complete

- [x] change to yaml format (JSON doesn't allow comments or multiline strings)
- [x] generate tex file
- [x] how to prevent duplicates?
- [x] construct vectors of pointers to nodes
- [x] construct tree from nodes recursively
- [x] sort vector of pointers to nodes
- [x] implement DFS for adding predecessors
  - [x] figure out how to add a node to `sorted_nodes`
- [x] add list of tables
- [x] add list of figures
- [x] definitions
- [x] theorems
- [x] proofs
- [x] detect cycles, prevent infinite loops
- [x] multiline proofs
- [x] remarks
- [x] show equation numbers only if they are referenced
- [x] listings
- [x] wiki_link
- [x] Put default preamble in separate source file
- [x] how many times to compile document?
- [x] add labels and references to envrionments
- [x] run bibtex
- [x] define (default) LaTeX preamble
- [x] define (default) LaTeX backmatter
- [x] define (default) LaTeX frontmatter
- [x] sources (BibTeX)
- [x] add --help command
- [ ] add options
  - [x] show wiki links
- [x] add timers to console output
- [x] automatically add label to Wikipedia search URL if `wiki` key is
      empty
- [x] Code base organization is less than ideal.
- [ ] ~~Support TOML?~~
- [x] Do not print anything in tex file for node labeled "//".

### Bugs

- [ ] implement fwd search for adding successors

### Functionality

- [ ] support unicode text
  - [ ] verify that tok writes unicode text
- [ ] Add keys that can be set as predecessors or successors in option
      flags
      - keys
        - [ ] `gen` generalization of
        - [ ] `case` special case of
        - [ ] `proc` procedure for computing a certain value
      - flags
        - [ ] put generalizations before/after special cases
        - [ ] "crib sheet" mode ignores keys
          - Automatically resize font to fit in page?
          - exclude
            - pretext
            - discussion
            - proofs
            - listings
            - examples
            - urls
          - include
            - procedures
            - algorithms
        - [ ] "reference" mode
          - ignores examples, discussion
          - includes motivation
        - [ ] exclude proofs (independent of "crib sheet" mode)
- [ ] `sources` key that appends bibtex format references to `.bib` file
- [ ] "presentation" mode?
- [ ] generate chapters, sections, subsections, etc.
- [ ] place references at end of each chapter
- [ ] appendix (all successor nodes that are not added to tree)

### LaTeX Formatting

- [ ] Option to select with sans serif font for computer screens and
      option to choose light/dark theme
- [x] add comments in tex file with yaml file names to mark item locations
- [ ] add option to print file names even if not generating a task list
- [ ] define LaTeX environment for types:
  - [ ] (what about proofs that require more advanced concepts?
        e.g. proof of Cramer's rule using Clifford Algebra)
  - [ ] history/intro
  - [ ] motivation
  - [x] examples (to go after text)
  - [ ] motivating_examples (to go before text)
  - [ ] algorithms https://www.overleaf.com/learn/latex/Algorithms
  - [ ] [best_practices, info, notes, cautions,
        warnings](https://tex.stackexchange.com/questions/21227/example-environment)
  - [ ] urls (as footnotes with hyperlinks)

### Interface

- [ ] Option: Exclude successors that are not predecessors
- [ ] Option: Put successors that are not predecessors in appendix
- [ ] Add option to load only files listed in command line for testing
      formatting of newly committed files.
- [ ] Add key for using `minted` package for listings
- [ ] Read all YAML files in a directory (loop, match)
- [ ] Finalize YAML format
- [ ] Ignore nonexistent predecessors, notify user
- [ ] Test with JSON files (as YAML is a superset of JSON)
- [ ] Declare Nested Nodes in YAML files
- [ ] add options
  - [ ] Select LaTeX engine
  - [ ] output file directory
  - [ ] output file name
  - [ ] user provided LaTeX preamble
  - [ ] exclude proofs
  - [ ] exclude exercises
  - [ ] include appendix (successors)
  - [ ] clean up generated files
- [ ]
### Documentation

- [ ] Available node types
- [ ] Describe how references to equations work
- [ ] Provide really good examples of acceptable YAML files
- [ ] Write guidelines for writing a good YAML file
- [ ] exercises (gather exercises from a section, and put them at the end)

### Markdown/Web

- [ ] generate md file
- [ ] compile md->tex->pdf using pandoc with raw_latex enabled
- [ ] bibliography

### Reference

- [Textbook Writing
  Tutorial](http://edutechwiki.unige.ch/en/Textbook_writing_tutorial)

### Add Later

- [ ] nowiki boolean key suppresses wiki links in case you know there is
      no Wikipedia page available; wiki key has no effect; root node’s
      nowiki set to false
- [ ] urls is a HashMap where key is display label and value is URL
      (recommended to include website name and page title in display
      label)
- [ ] sec key allows custom section/chapter heading title for a
      particular node if that node is determined to require a section
      heading
- [ ] Change disc to post, pretext to pre
- [ ] analogy key serves to separate analogies from the rest of text,
      suppressed by default
- [ ] q is a sequence (Vec<String>) storing questions for author to
      answer in future drafts
- [ ] ex and eg are not types; they are keys
- [ ] sol key for solutions
- [ ] alg, alg2e, and algx for different algorithms environments
- [ ] mint for different listings environment
- [ ] sources is a sequence of sources in BibTeX format (need to
      eliminate duplicates in .bib file)
- [ ] If wiki is blank, change display label to “Search Wikipedia”
- [ ] cap is caption
- [ ] sub is subset of
- [ ] super is superset of
- [ ] Keys for authors (given and surnames); sort alphabetically and
      remove duplicates after collecting nodes

Options:

- [ ] Include wiki links
- [ ] Exclude pre and post
- [ ] Show questions
- [ ] Reverse branch sorting
- [ ] Exclude examples
- [ ] Exclude exercises
- [ ] Exclude solutions
- [ ] Successor depth
- [ ] Exclude analogies
- [ ] Exclude successors (include by default); expect missing references
      in pdf

Changes:

- [ ] Show labels even if env not specified
- [ ] Pass immutable reference to options struct between functions

- [ ] Additions with no plan for implementation:
- [ ] List of acronyms
- [ ] How to suppress list of * when empty (LaTeX)?
- [ ] How to make sure a figure/listing appears (immediately) after it
      is referenced (so that listings can be declared as predecessors
      and they don’t end up at the end of the book or missing completely
      because they’re declared as successors)?
