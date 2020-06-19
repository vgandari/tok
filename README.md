# tok

## Introduction

`tok` (stands for Tree of Knowledge) is a command line tool that
**optimally sorts topics** to and **generates a document** with those
topics in order in PDF format.
Each topic is stored in its own YAML file, which also contains
dependency information and metadata (e.g. formatting).

`tok` reads each YAML file, constructing a graph where each node
corresponds to a YAML file, and uses the graph to sort the nodes in a
way that **does not violate dependency relationships**.
`tok` then generates a TEX file with text from all nodes sorted and
formatted as specified, and by default calls a LaTeX engine to generate
a PDF (use `--no-pdf` to see the sorted topics in the terminal without
generating a PDF).

### Goals

`tok` aims to solve problems for both readers (students) and authors
(instructors).
The goals of `tok` are as follows

- **Organize notes automatically** so that they may serve as a **quick,
  but comprehensive reference**
- Provide readers with a way to **customize organization of a document
  while respecting the author-specified dependency relationships between
  topics**.
- **Unburden readers from the task of revisiting information** presented
  earlier/later in a text in order to understand information presented
  later/earlier; i.e., produce a **text designed to be read and studied
  sequentially**.
- Allow **authors to focus on individual topics in isolation** and
  remove the burden of finding the optimal order in which to present
  topics to readers.
- Provide readers with a way to **strip away unnecessary information**
  leading up to a topic of interest while presenting all necessary
  information.
  By stripping away unneccesary information and maintaining an optimal
  ordering of topics, the **reader no longer has to "hunt" for relevant
  information**.

## Install

`tok` requires Rust and the `cargo` utility.
Install Rust and `cargo` from
[here](https://www.rust-lang.org/tools/install).

If you would like to install `tok` to your PATH, run

```sh
$ cargo install --path .
```

from the `tok` project root.
By default, `cargo` will install `tok` in `$HOME/.cargo/bin`.
Make sure this is in your PATH.
If you configured `cargo` to install binaries somewhere else, make sure
that directory is in your PATH.

## Usage

See `tok --help` for command line options.
Options marked `[INOP]` are inoperable and have no effect.

At the moment, `tok` is hardcoded to use `xelatex` to compile PDFs.
If you would like to use a different LaTeX engine, you can still process
the generated TEX file on your own using that LaTeX engine.

To include _all_ files in your document's project directory (*nix
systems), run

```sh
tok `find . -name '*.yaml' -print`
```

### Structuring a Project for `tok`

```
project/
|-code/               <-- for code listings
|-images/             <-- for figures
|-output/             <-- all output will go here; see .gitignore
|-texinput/
|  |-backmatter.tex   <-- optional
|  |-frontmatter.tex  <-- optional
|  |-preamble.tex     <-- optional
|-yaml/               <-- location of yaml files; can be any name;
                          also where the tok command is run
```

If using Git as your SCM, include the following in your `.gitignore`:

```
!code/
!images/
output/
!texinput/preamble.tex
!texinput/frontmatter.tex
!texinput/backmatter.tex
```

The output of `tok` and `xelatex` will be under a directory called
`../output` relative to where `tok` was run (e.g. if running `tok` in
`project/yaml/` above, the outout will be located in `project/output/`).

### Defining a Dependency Graph

Each YAML file in your project represents a node.
`tok` reads these YAML files and constructs a graph based on the
dependency relationships between the nodes.
Dependency relationships between nodes are declared within YAML files.
The following keys are for expressing dependency relationships:

 - `after`: Text stored in this node must appear "after" all text
   contained in the files in this list.
 - `before`: Text stored in this node must appear "before" all text
   contained in the files in this list.

For example, if node A depends on node B (i.e. text stored in A must
appear later in the document than text stroed in B), then you may write

```yaml
after:
  - B.yaml
```

inside of `A.yaml`, or

```yaml
before:
  - A.yaml
```

inside of `B.yaml`, or _both_.

Note that if a cycle happens to form within the graph you define (for
larger projects, it's hard not to end up with any cycles!), then a
directed acyclic graph will still be generated (i.e. `tok` will
terminate even if it detects cycles), but there will be a difference in
how the nodes are ordered within your document.

### Adding Content to a Document

This section describes how to add content corresponding to each node in
your document.
Each node is represented by a YAML file from which `tok` loads data.

The following keys are available for storing information that appears in
the PDF:

 - `pre`: Text for introducing the main text, outside of any LaTeX
   environment.
 - `main`: The main text that appears inside your environment
 - `post`: Text for providing more discussion after the main text,
   outside of any LaTeX environment.
 - `pfs`: If using the `thm`, `lem`, `cor`, or `rem` environments,
   include one or more proofs
 - `urls`: Any URLs that might be helpful
 - `eli5`: "Explain like I'm five", a simple explanation, even if not
   technically correct.
 - `src`: BibTeX items; `tok` will automatically generate a BIB file
   free of duplicates

> NOTE: All keys must contain valid LaTeX code.

> NOTE: When using the `-c` option ("crib sheet"), text in `pre` and
> `post` will not appear n the document.
> It is worth reviewing the command line options for customizing the
> generated document.

Declaring a LaTeX environment for `main` text is possible by prefixing
the file name.
File names should use the underscore `_` as a separator between words.
The prefix before the first underscore determines the LaTeX environment.
The rest of the file name determines the title that appears in that
environment.

`tok` recognizes the following environment prefixes.

- `def` - definition
- `thm` - theorem
- `cor` - corollary
- `lem` - lemma
- `plain` - plain text, show title in bold before any text from this
  node
- `x` - plain text, hide title, set `nowiki: true`
- `task` - plain text, show title in bold before any text from this
  node, show "TASK" with empty checkbox in left margin
- `done` - plain text, show title in bold before any text from this
  node, show "DONE" with an "X" in left margin

> NOTE: It is not recommended to create a document that includes `task`
> and `done` along with any other environments.

> NOTE: It is not recommended to overwrite the `env` key within a YAML
> file.

If an environment is selected so that the title/label appears in the
text, underscores are replaced with spaces and the title/label is
converted to title case according to [these
rules](https://daringfireball.net/2008/05/title_case).
You can overwrite the `label` key directly in the YAML file if these
rules do not suit your purpose.

If you do not want your text to appear in a LaTeX environment, use
`x` or `plain` as a prefix.
The difference between `plain` and `x` is that `plain` will result in
the label appearing in bold in the document, while `x` will not print
the label at all.
Another difference is that `x` specifically disables adding a link to
Wikipedia's search page for that topic, even when the `-w` option is
used.

Wikipedia's search page is convenient because if there is an exact match
between the search term and an existing page title, Wikipedia
automatically redirects to that page.
This is useful for topics with very specific headings that do not have
Wikipedia pages, or if you don't bother adding a specific link to your
node, a reader can still find a related page on Wikipedia without
manually copying and pasting text.
