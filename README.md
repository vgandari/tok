# tok

## Introduction

`tok` (stands for Tree of Knowledge) is a command line tool that
generates a PDF document (e.g. a textbook) based on a tree of topics.
Each node in the tree represents a topic.
The text for each topic (i.e. the text that is printed tot he document)
is stored in a YAML file.
Each YAML file also contains metadata such as formatting and dependency information.
The input to `tok` is a YAML file or set of YAML files containing text
for topics.
`tok` uses the dependency information in the input files to construct a
tree of topics, load data from the files required to construct the tree,
sort the topics, generate a TEX file, and call LaTeX to generate a PDF
from the TEX file.
The text contained within the input files appears towards the end of the
document, with all their dependencies appearing at the beginning of
the document.

### Goals

`tok` aims to solve problems for both readers (students) and authors
(instructors).
The goals of `tok` are as follows

- Develop a standard method of organizing notes into a textbook-like
  format, with an optimal ordering of the topics presented
- Provide readers with a way to customize the ordering of topics
  presented in a document while respecting the author-specified
  dependency information between topics.
- Unburden readers from the task of revisiting information presented
  earlier/later in a text in order to understand information presented
  later/earlier; i.e., produce a text designed to be read and studied
  sequentially.
- Allow authors to focus on individual topics in isolation
  and remove the burden of finding the optimal order in which to present
  topics.
- By isolating topics and declaring dependencies, reveal to authors
  sources of ambiguity and relative position of a topic relative to
  other topics within the final document.
- Provide readers with a way to strip away unnecessary information
  leading up to a topic of interest while presenting all necessary
  information.
  By stripping away unneccesary information and maintaining an optimal
  ordering of topics, the reader no longer has to "hunt" for relevant
  information.

## Build

`tok` requires Rust and the `cargo` utility.
Install Rust and `cargo` from
[here](https://www.rust-lang.org/tools/install).
From the command line, run

```sh
$ cargo build --release
```

from the `tok` project root.

## Install

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

At the moment, `tok` is hardcoded to use `xelatex` to compile PDFs.

If you would like to use a different LaTeX engine, you can still process
the generated TEX file on your own using that LaTeX engine.

### Structuring a Project for `tok`

```
project/
|-code/               <-- for code listings
|-images/             <-- for figures
|-output/             <-- all output will go here; do not touch!
|-texinput/
|  |-backmatter.tex   <-- optional
|  |-frontmatter.tex  <-- optional
|  |-preamble.tex     <-- optional
|-yaml/               <-- location of yaml files; can be any name;
                          also where the tok command is run
```

In your `.gitignore`, include the following

```
!code/
!images/
output/
!texinput/preamble.tex
!texinput/frontmatter.tex
!texinput/backmatter.tex
```

The output of `tok` and `xelatex` will be under a directory called
`output` relative to where `tok` was run.

### Creating and Writing YAML Files

This section describes how to create a YAML file from which `tok` will
load data.
First, `tok` uses a naming convention for placing text in a LaTeX
environment in a conveinent way.
File names should use the underscore `_` as a separator between words.
The prefix before the first underscore determines the LaTeX environment.
The rest of the file name determines the title that appears in that
environment.
Titles appear in title case according to [these rules]().
If you do not want your text to appear in a LaTeX environment, use
`x` or `plain` as a prefix.
The difference between `plain` and `x` is that `plain` will result in
the label appearing in bold in the text, while `x` will not print the
label at all.
Another difference is that `x` specifically disables adding a link to
Wikipedia's search page for that topic, even when the `-w` option is
used.
A nice thing about Wikipedia's search page is that if there is an exact
match between the search term and an existing page title, Wikipedia
automatically redirects to that page.
This is useful for topics with very specific headings that do not have
Wikipedia pages.

`tok` recognizes the following environment prefixes.

- `def` - definition
- `thm` - theorem
- `cor` - corollary
- `lem` - lemma

In addition to generating a tree of "knowledge", `tok` can generate a to
do list using the `task` and `done` file prefixes.

- [ ] Complete environments list

In addition to parsing the file name itself, `tok` reads the YAML keys
in each file.
First, the `env`, `label`, and `nowiki` keys may be overwritten,
regardless of the filename. This is allowed, but not recommended.
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

The following keys are for expressing dependency relationships:

 - `after`: all nodes in this list must come "after" text contained in
   this file.
 - `before`: all nodes in this list must come "before" text contained in
   this file. (This is a source of bugs, so try not to use this for
   now.)

### Command Line Options

```
$ tok --help
tok 1.0
Victor Gandarillas
Tree of Knowledge -- textbook and to do list generator

USAGE:
    tok [FLAGS] [OPTIONS] <FILES>...

FLAGS:
    -C, --crib           [INOP] "Crib sheet" or "Cheat sheet" mode
                         (overrides -a or --all flag; default is "full text" mode)
    -d, --draft          [INOP] Include "DRAFT" watermark (default is no
                         watermark)
        --eli5           Include simple explanations/"Explain Like I'm Five"; OFF by default
    -e, --engine         [INOP] Choose LaTeX engine (default is xelatex)
    -g, --examples       [INOP] Include examples in textbook (default is hidden)
    -x, --exercises      [INOP] Show exercises
        --gen-before     [INOP] Treat paths in `gen` sequence as predecessors and
                         paths in case sequence as successors (default is
                         opposite)
    -h, --help           Prints help information
    -P, --preamble       [INOP] Choose preamble file
    -p, --proofs         Hide proofs
    -q, --questions      Show questions for author to answer in a future draft (default
                         is hidden)
    -r, --reverse        Reverses branch sorting better suited for task lists
    -u, --url            Show links to URLs
    -V, --version        Prints version information
    -w, --wiki           Show wikipedia links (does not show for nodes where
                         nowiki key takes on true value; default is hidden)
        --no-appendix    Do not include "Appendix" heading
    -y, --yaml           Show YAML file name in PDF

OPTIONS:
        --author <author>    Set author of document generated
    -c, --config <FILE>      [INOP] read from config file specifying formatting
                             options (default is .tok-config in same directory)
        --depth <depth>      Depth of successor branch; default is to load all
                             successors, but placing a limit may reduce the size
                             of a document; setting --depth="0" guarantees that no
                             appendix is generated; a negative number leads to
                             default behavior
        --output <output>    Path to output PDF, default is ../output/main.pdf
        --title <title>      Set title for document generated

ARGS:
    <FILES>...    Files to read
```

Options labeled `[INOP]` are currently inoperable and have no effect on
the output.

### Building a PDF from YAML Files

From the command line, running

```sh
$ tok A.yaml B.yaml
```

will generate a PDF covering all the material declared as a direct
dependency of `A` and `B`, as well as their dependencies, and so on.
This example assumes that you are in the same directory as the files
`A.yaml` and `B.yaml`.
Note that a tree can be arbitrarily large, even with a single file as
input, depending on the dependency graph.

Note that one of `A` and `B` may also depend on the other, which will
affect the order in which they appear, and how far apart they appear in
the generated textbook.

To include _all_ files in your document's project directory, run

```sh
tok `find . -name '*.yaml' -print`
```

in your document's project directory.

### Conventions (LaTeX)

Use `\eqref` for referencing equations.
This will provide a link to the equation in the generated PDF.

Name equations using `\label{eq:<path>__<name>}`, replacing `<path>`
with the path to the current file, and `<name>` with the name you would
like to use for this equation.
