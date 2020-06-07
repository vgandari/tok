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
- [x] add timers to console output
- [x] automatically add label to Wikipedia search URL if `wiki` key is
      empty
- [x] Code base organization is less than ideal.
- [x] Do not print anything in tex file for node labeled "//".
- [ ] add options
  - [x] show wiki links
  - [x] Reverse branch sorting
  - [x] Include wiki links
  - [x] Show questions
  - [x] implement fwd search for adding successors
  - [x] successor depth
    - [ ] pass value instead of string from command line
  - [x] urls ~~(as footnotes with hyperlinks)~~
- [x] ~~add comments in tex file with yaml file names to mark item locations~~
- [x] Pass immutable reference to options struct between functions
      answer in future drafts
- [x] q is a sequence (Vec<String>) storing questions for author to
- [x] refs is a sequence of sources in BibTeX format (need to
      eliminate duplicates in .bib file)
- [x] add option to print file names even if not generating a task list
- [x] `refs` key that appends bibtex format references to `.bib` file
- [x] appendix (all successor nodes that are not added to tree)
- [x] Infer `env`, `label`, ~~`wiki`~~ from file name; ~~maybe `nowiki`
      as well (add a 0 to the end of the file name?)~~
- [x] Set title, author from command line
- [x] Set nowiki in filename
- [x] Add --no-appendix option
- [x] Add output option
- [x] eli5: Explain Like I'm Five, for simple, informal explanations and
      analogies; good for introductory texts
- [x] Enable organizing nodes in sub directories

### Bugs

- [ ] `before` nodes are borrowed twice, resulting in panic
- [ ] If two nodes are listed as 'before' each other, they don't appear.
- [ ] Questions list is duplicated
- [ ] Whitespace for Wikipedia links and URLs is weird
- [ ] Add environments structure for pattern matching?

### Functionality

- [ ] generate chapters, sections, subsections, etc.
- [ ] place references at end of each chapter
- [ ] display which paths can be traversed in parallel
- [ ] support unicode text
  - [ ] verify that tok writes unicode text
- [ ] Add keys that can be set as predecessors or successors in option
      flags
      - keys
        - [ ] "not to be confused with" key
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
- [ ] "presentation" mode?

### LaTeX Formatting

- [ ] Option to select with sans serif font for computer screens and
      option to choose light/dark theme
- [ ] define LaTeX environment for types:
  - [ ] (what about proofs that require more advanced concepts?
        e.g. proof of Cramer's rule using Clifford Algebra)
  - [ ] history/intro
  - [ ] motivation
  - [ ] examples (as separate nodes; to go after text)
  - [ ] motivating_examples (to go before text)
  - [ ] algorithms https://www.overleaf.com/learn/latex/Algorithms
  - [ ] [best_practices, info, notes, cautions,
        warnings](https://tex.stackexchange.com/questions/21227/example-environment)

### Interface

- [ ] Add `--no-pdf` option
- [ ] Config file (`.tok` in document project root)
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
- [ ] Electron app to visually create files and draw dependency
      relationships

### Documentation

- [ ] Create GitHub page
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

### Keys to Add Later

- [ ] why: include motivation for introducing current topic; keeps
      motivation and discussion separate
- [ ] sec: Section title if section is generated (different from label)
- [ ] nowiki boolean key suppresses wiki links in case you know there is
      no Wikipedia page available; wiki key has no effect; root node's
      nowiki set to false
- [ ] urls: HashMap where key is display label and value is URL
      (recommended to include website name and page title in display
      label)
- [ ] sec key allows custom section/chapter heading title for a
      particular node if that node is determined to require a section
      heading
- [ ] ex and eg are not types; they are keys
- [ ] sol key for solutions
- [ ] alg, alg2e, and algx for different algorithms environments
- [ ] ~~mint for different listings environment~~
- [ ] cap is caption
- [ ] sub is subset of
- [ ] super is superset of
- [ ] Keys for authors (given and surnames); sort alphabetically and
      remove duplicates after collecting nodes

Changes:

- [ ] Additions with no plan for implementation:
- [ ] List of acronyms
- [ ] How to suppress list of * when empty (LaTeX)?
- [ ] How to make sure a figure/listing appears (immediately) after it
      is referenced (so that listings can be declared as predecessors
      and they don’t end up at the end of the book or missing completely
      because they’re declared as successors)?
