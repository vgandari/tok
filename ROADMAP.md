# ROADMAP

## Next Steps

- [ ] Flag problematic deadlines
  - set flag indicating if any children have later deadline
    - start at root, go to leaf nodes, check if any children have later
      deadline than parent, set flag in parent indicating that a child
      has a later deadline if child has later deadline, or child flag
      set to true
    - if flag is set to true, show deadline of this node in
      red
- [ ] Update README with tasks vs topics, and deadline keys
- [ ] Put references at end of chapter (or document if max heading depth
      < 3)
- [ ] Add examples as immediate successors
  - Topic has `examples: Vec<Node<?>>` field
  - print examples in `write_tex` function
- [ ] Add exercises at end of chapters
  - only if chapters are generated
  - Topic has `exercises: Vec<Node<?>>` field
  - gather and print exercises in `write_tex` function
  - set flag indicating that all nodes in a chapter lack exercises to
    avoid unnecessary loop over nodes
- [ ] Allow for changing dependency relationships (generalizations)
- [ ] `hist` key for providing historical context, not necessary for
      technical understanding
- [ ] Compile data from predicted vs actual duration and export to csv,
      perform analysis on data in separate tool
- [ ] Add `scale` key for scaling tree cost for a node, to push it
      towards the beginning/end of a document.
- [ ] Distinguish between `Task` and `Topic` nodes in `tex.rs`, but not
      elsewhere. Use Traits?
      - Create node based on whether node is topic or task
      - Compute costs based on duration, float/wait time, and deadline
        - requires signed types
- [ ] Create symlinks instead of copying images and code directories
  - [x] Mac/UNIX
  - [ ] Linux
  - [ ] Windows
- [ ] place references at end of each chapter, if any chapters are generated
  - if max heading depth <= 3 (no chapters), then print
    `\begin{refsection}`, `\end{refsection}` at begining and end of
    document.
  - default backmatter is empty.
- [ ] Reimplement heading generation as another tree?
- [ ] Include exercises at end of chapter, similar to how examples are
      immediate successors

### Paper/Article Output

- [ ] Force section to be made using `sec`/`ch` value for `env`
- [ ] Flag to limit heading generation to sections (no chapters, parts,
      etc.), for generating headings for conference papers and journal
      articles within sections defined using `sec` values
- [ ] Flag to suppress heading generation to only generate subsections

## Headings

- [ ] Only insert heading with increased heading depth if there are at
      least two headings with the increased heading depth (e.g. do not
      insert a chapter heading if there is only one chapter; do not
      insert any section headings if there is only one section within a
      chapter; do not generate subsection if there are no sections)
- [ ] Do not make headings for items in appendix before start of
      appendix
- [ ] If book headings are inserted, generate an appendix per book
- [ ] If chapter heading generated after appendix, use the appendix
      chapter style

### Bugs

- [ ] Incorporate deadlines for task nodes (see FIXME in `topic.rs`)
- [ ] Do not insert a section heading if the depth increases by 2
      instead of 1 (e.g. do not generate subsection if there are no
      sections)

## Functionality

- [ ] `ext` (external) key that pulls text from websites (e.g.
      [nLab](https://ncatlab.org/nlab/show/HomePage)), gives proper
      credit, and saves user from writing content; more powerful than
      `urls` key
- [ ] Print "Related Concepts" at end of topic where all the labels for
      topics under `incl` key are listed (with links)
- [ ] `rel` key for related topics that have no dependency relationship
      with current topic (e.g. ?)
- [ ] Executable/verifiable code?
- [ ] `sec` env forces node to BEGIN a section
- [ ] `gen` and `case` with option to set ordering (default `case` nodes
      precede `gen` nodes)
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

- [ ] Test with JSON files (as YAML is a superset of JSON)
- [ ] add options
  - [ ] output file directory
  - [ ] output file name
  - [ ] exclude exercises
- [ ] Electron app to visually create files and draw dependency
      relationships

## Documentation

- [ ] Write guidelines for writing a good YAML file
- [ ] exercises (gather exercises from a section, and put them at the end)

## Markdown/Web

- [ ] generate md file, use pandoc filters; render html
- [ ] default CSS

## Keys to Add Later

- [ ] `sec` key allows custom section/chapter heading title for a
      particular node if that node is determined to require a section
      heading
- [ ] ex and eg are not types; they are keys
- [ ] sol key for solutions
- [ ] alg, alg2e, and algx for different algorithms environments
- [ ] Keys for authors (given and surnames); sort alphabetically and
      remove duplicates after collecting nodes

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
  - [ ] successor depth
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
- [ ] ~~why: include motivation for introducing current topic; keeps
      motivation and discussion separate~~
- [ ] ~~Publish docs on GitHub Pages (not helpful since this isn't a
      library)~~
- [ ] ~~Default preamble: choose book or article based on sections
      generated~~ Default is memoir class; article option does this for
      you
- [x] User option to select LaTeX engine
- [x] Provide really good examples of acceptable YAML files
- [x] Add `aka` key
- [x] Add start and completion date (list of `usize` for little endian
      dates or `None` for not started/incomplete) for tasks, keep
      `done` prefix for tasks with unknown completion dates; task is
      marked done if it is `done` or `task` with completion date; task
      displays duration iff it has start and completion date; task
      displays expected duration if incomplete; show
      start and completion dates after label
- [x] Compute cost based on actual duration; if incomplete, compute cost
      based on expected duration
- [x] Require prefix for files to specify environment (panic if no prefix)
- [x] Take deadlines into account
  - nodes with earlier deadlines appear before nodes with later deadlines
    - sort nodes by deadline
      - starting with node with earliest deadline, run topological sort
        - don't reintroduce a child of a node with an earlier deadline
          - inside topological sort, if node is in sorted_nodes (bool
            value in Node<T>), then skip
        - don't add a child with a later deadline after its parent
          - if node is in sorted_nodes (bool value in Node<T>), then don't
            run topological sort on that node again
  - nodes without deadlines appear after nodes with deadlines
    - after running topological sort on nodes with deadlines, run topological
      sort on root node, and do not insert nodes that are already in
      sorted_nodes
