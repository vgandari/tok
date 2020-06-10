# tok

## Introduction

`tok` (stands for Tree of Knowledge) is a command line tool that
generates a PDF document (e.g. a textbook) based on a tree of topics.
The tree is defined in a set of YAML files containing the text covering
some topic that should be included in the document, dependency
information, and other data an author may include for convenience.
The input to `tok` is a file or set of files containing text for topics
and their dependency information.
`tok` uses the dependency information in the input files to construct a
tree of topics, load data from the files required to construct the tree,
sort the topics, generate a TEX file, and call LaTeX to generate a PDF
from the TEX file.


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

If you would like to learn how `tok` works, see
[How It Works](#how-tok-works) below.
You can try out `tok` on an existing repository
[here](https://github.com/vgandari/everything).

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

### YAML Files

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

## Building the Documentation

- [ ] TODO: provide a place for visitors to view documentation without
      needing to build it themselves.
### Conventions (LaTeX)

Use `\eqref` for referencing equations.
This will provide a link to the equation in the generated PDF.

Name equations using `\label{eq:<path>__<name>}`, replacing `<path>`
with the path to the current file, and `<name>` with the name you would
like to use for this equation.

### Specific to `tok`

- Don't put everything in one file.
  `tok` shines when topics are split into many files.
  The point of splitting topics into files is so that authors can focus
  on content rather than organization -- let `tok` handle that.
- By isolating each topic to a single file, an author can focus
  on a very small section of text, reducing the likelihood of assuming
  that the reader will be more familiar with ideas mentioned than they
  are.

### General Writing Tips

- Don't try to sound smart; make your notes/book accessible.
  Use the smallest/most common word possible, but no smaller/more
  common than that.
  Simplify your sentences.
  Express few ideas at once.
  This also helps split topics into several files, giving `tok` more
  power to organize your document!
- A well formed sentence is the result of a well formed thought.
  Once you get your ideas on the page, review each sentence and make
  sure it expresses exactly what you are thinking (no more, no less;
  leave no room for ambiguity).

## How `tok` Works

The following sections describe how `tok` works at a high level.
That being said, there's a lot of detail to cover that may be best
illustrated by an example.
I recommend visiting my "book",
["Everything...Or At Least Some of It"](https://github.com/vgandari/everything),
prior to reviewing the following sections.

### Overview

The author writes about several topics.
Each topic is contained in its own YAML file.
Each YAML file also contains other metadata, including dependency
information.

The following example doesn't explain how to _use_ `tok`, only how to
_conceptualize_ what `tok` does.

- [ ] TODO: Add some figures

For example, let's say we write about topics `A`, `B`, `C`, and `D`.
Then we have files `A.yaml`, `B.yaml`, `C.yaml`, and `D.yaml`.
We can then express dependency information.
Let's say topic `D` mentions a definition introduced in topic `C`.
We would then express that `D` has to appear _after_ topic `C`.
We include `C.yaml` in the list of files that appear under the `after`
key in `D.yaml`.
If we run `tok D.yaml`, we will have a document that contains text
stored in `C.yaml`, followed by text stored in `D.yaml`.
If we want to include `A` and `B`, we will either need to express their
dependence on `C` or `D` (or vice versa, depending on what `A` and `B`
are), or we can add their files to the list of inputs like so:
`tok A.yaml B.yaml D.yaml`.
The order of the input file names does not affect the order of the
content in the output document.
This is a simple example, and `tok` can handle far more complex document
structures.

`tok` reads from YAML files that each store information for a topic of
interest, selects the LaTeX environment indicated in the YAML
file, and sorts all topics before generating a TEX file based on
the dependency relationships expressed within the YAML files.
`tok` then compiles the TEX file into a PDF.

Each YAML file also contains information about its dependencies (other
YAML files) and which LaTeX environment to use.

The generated textbook is meant to be read in a purely sequential manner
(assuming the dependency relationships are defined correctly and
completely in all the YAML files).

### How `tok` Sorts Topics

`tok` arranges topics into a tree.
Each file is a node.
Dependency declarations are directed edges.
Edges are directed from successor to predecessor, in reverse order of
how topics are to appear in the document (a topic appearing in
the document body -> a topic appearing before).
The root node is an empty node; it does not add any text to the
final document.
The root node corresponds to the end of the document.
The input files are predecessors of the root node.
That is, the final document culminates in the topics represented by the
input files (there is an exception described later, which results in
generating an appendix, but this holds for all text prior to the
generated appendix).

Visualizing this tree as a set of nodes connected by directed edges
(you can draw dots and arrows to represent nodes and edges,
respectively), define a leaf node as a node with no outbound egdes.
A branch is a path from a node to a leaf node.
Some branches may result in cycles.
`tok` detects cycles and stops adding nodes to a branch upon detecting a
cycle.
`tok` computes the "cost to go" from each node to the root (traversing
the tree in the direction opposite of the edges) and sorts the branches
by cost.
`tok` then uses a
[Depth First Search](https://en.wikipedia.org/wiki/Depth-first_search),
modified to account for multiple branches arriving at the same node,
to sort the topics.

The reader can customize the order in which topics are sorted by
reversing the order of the branches.

The default order arranges topics for the reader to follow the "critical
path"; the path with the highest "cost to go".
This keeps the reader on the longest path to the terminal topic,
switching branches as necessary.

The reverse order arranges topics for the reader to encounter the
"lowest hanging fruit"; the path with the lowest "cost to go".
This keeps the reader on the shortest path to the terminal topic,
switching branches as necessary.

If topics are a set of tasks, the reverse order may be suitable for
manging a project, where completing tasks considered "low hanging fruit"
first is a better strategy to make/show progress early in a project's
lifetime, as larger tasks are typically larger sources of delay.

(For more information about supported YAML keys, see the section on
[Writing YAML Files](#writing-yaml-files).)

Finally, because, each topic is (ideally) written independent of
context, `tok` allows a user (reader) to customize the organization of
the textbook according to their learning style using the `-r` option
without changing the dependency relationships defined in the YAML files'
`after` and `before` keys.

(For more information about supported command line options, see the
section on [Usage](#usage).)

## Contributing

Feel free to open an issue or submit a pull request.

## Integrating `tok` with your text editor

### VSCode

In VSCode, use the
[Run on Save](https://marketplace.visualstudio.com/items?itemName=emeraldwalk.RunOnSave)
extension and add the following to `settings.json`.
Upon saving a YAML file, this will pass that YAML file as the only
argument to `tok`, compile a PDF with URLs, Wikipedia links, questions
for the author, and corresponding YAML file names included in the PDF.

```json
"emeraldwalk.runonsave": {
    "commands": [
        {
            "match": "\\.yaml$",
            "cmd": "cd ${workspaceFolder}/yaml/ && tok -uywq ${fileBasename}"
        }
    ]
},
```
