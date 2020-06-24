# ROADMAP

## Complete

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
- [x] Add `no-pdf` option
- [x] Ensure tree is generated, as opposed to DAG
- [x] Add `--no-pdf` option
- [x] Generate TEX file, but not PDF when `--no-pdf` is used
- [x] Change Wikipedia links to "Search for ``<label>'' on Wikipedia"
- [ ] ~~Config file (`.tok` in document project root)~~
- [ ] ~~Option: Exclude successors that are not predecessors~~
- [x] Read all YAML files in a directory (loop, match)
- [x] Finalize YAML format
- [ ] ~~Ignore nonexistent predecessors, notify user~~
- [ ] ~~Add key for using `minted` package for listings~~
- [ ] ~~display which paths can be traversed in parallel~~
- [x] generate chapters, sections, subsections, etc.

## Next Steps

- [ ] Create symlinks instead of copying images and code directories
  - [ ] Mac/UNIX
  - [ ] Linux
  - [ ] Windows
- [ ] Use directory name where tok is run as name of TEX and PDF output
      file names

### Paper/Article Output

- [ ] Force section to be made using `sec`/`ch` value for `env`
- [ ] Flag to limit heading generation to sections (no chapters, parts,
      etc.), for generating headings for conference papers and journal
      articles within sections defined using `sec` values
- [ ] Flag to suppress heading generation to only generate subsections
- [ ] place references at end of each chapter, if any chapters are generated
  - if max heading depth <= 3 (no chapters), then print
    `\begin{refsection}`, `\end{refsection}` at begining and end of
    document.
  - default backmatter is empty.

## Functionality

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

## LaTeX Formatting

- [ ] Default frontmatter/preamble: Hide lists of figures and tables if empty
- [ ] Default preamble: choose book or article based on sections generated
- [ ] Hide appendix if there are no nodes in appendix
- [ ] Generate section headings within appendix
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
- [ ] Nomenclature and list of acronyms

## Interface

- [ ] Add option to load only files listed in command line for testing
      formatting of newly committed files.
- [ ] Test with JSON files (as YAML is a superset of JSON)
- [ ] add options
  - [ ] Select LaTeX engine
  - [ ] output file directory
  - [ ] output file name
  - [ ] exclude exercises
  - [ ] clean up generated files
- [ ] Electron app to visually create files and draw dependency
      relationships

## Documentation

- [ ] Create GitHub page
- [ ] Provide really good examples of acceptable YAML files
- [ ] Write guidelines for writing a good YAML file
- [ ] exercises (gather exercises from a section, and put them at the end)

## Markdown/Web

- [ ] generate md file

## Keys to Add Later

- [ ] why: include motivation for introducing current topic; keeps
      motivation and discussion separate
- [ ] nowiki boolean key suppresses wiki links in case you know there is
      no Wikipedia page available; wiki key has no effect; root node's
      nowiki set to false
- [ ] sec key allows custom section/chapter heading title for a
      particular node if that node is determined to require a section
      heading
- [ ] ex and eg are not types; they are keys
- [ ] sol key for solutions
- [ ] alg, alg2e, and algx for different algorithms environments
- [ ] Keys for authors (given and surnames); sort alphabetically and
      remove duplicates after collecting nodes
