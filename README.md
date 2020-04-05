# tok

## Introduction

`tok` (stands for Tree of Knowledge) is a command line tool for
compiling documents from a set of files.
These files contain document text and dependency information specifying
the ordering of text contained within files.
The dependency information is used to sort the files as specified by the
user.

### Motivation

- I wanted a way to organize my notes in a way that I could refer to
  a specific subset of notes without constructing a mental map of
  topics (that could be incomplete) every time I go back and review a
  topic.
- Explanations for advanced concepts are often highly dependent on
  context. Especially at advanced levels, this provides little
  motivation/flexibility for authors to provide self-contained
  explanations. The use of multiple files with their relationships to
  other files expressed within them forces an author to at least
  consider providing self-contained explanations, and allows
  collaborators to more easily identify ways to clarify text.
- In some cases, publishers are unable to review advanced texts
  properly, possibly providing the author with feedback from an
  inappropriate audience.

### Goals

The goals of `tok` are as follows

- Textbook organiztion is not standardized or personalized. `tok` aims
  to solve both of these problems.
- Allow authors to focus on individual topics in isolation
  and remove the burden of finding the optimal order in which to present
  topics.
- By isolating topics, reveal to authors sources of ambiguity and
  information lacking in their writing.
- Provide readers with a way to strip away unnecessary information
  leading up to a topic of interest while presenting all necessary
  information.
- Provide readers with a way to customize the ordering of topics
  presented in a document while respecting the author-specified
  dependency information between topics.

### Target Audience

This tool is for

- Anyone who wants to organize their notes in a more logical manner
  without having to search for that one thing they remember seeing in
  the book/their notes, but can't remember where it was.
- Anyone interested in writing a textbook for publication, but may be
  unsure (or overconfident) in their ability to organize their textbook
  for students and just want to get their thoughts on paper.
- At the moment, this is more for a technical audience, but I'm open to
  feedback from anyone in the humanities as well, especially when it
  comes to adding support for additional YAML keys.

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

If you would like to learn how `tok` works, see [How It Works]() below.
You can try out `tok` on an existing repository
[here](https://github.com/vgandari/everything).

At the moment, `tok` is hardcoded to use `xelatex` to compile PDFs.

If you would like to use a different LaTeX engine, you can process the
generated TEX file on your own using that LaTeX engine.

The output of `tok` and `xelatex` will be under a directory called
`output` relative to where `tok` was run.

### Command Line Options

You can view the various options by

```sh
$ tok --help
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

Note that one of `A` and `B` may also depend on the other, which will
affect the order in which they appear, and how far apart they appear in
the generated textbook.

All the files must be in the same directory, and the command must be
executed in that directory.

To include all files in your document's project directory, run

```sh
tok `find . -name '*.yaml' -print`
```

in your document's project directory.

The output will be `./output/main.pdf`.
A document generated using this command will not contain an appendix.

You can customize the preamble, frontmatter, and backmatter in
`./texinput/preamble.tex`, `./texinput/frontmatter.tex`, and
`./texinput/backmatter.tex`, respectively.

You can also modify `./output/main.tex` directly and reprocess it using
the LaTeX engine of your choosing to generate a PDF if `xelatex` does
not produce the desired output.

## Building the Documentation

- [ ] TODO: provide a place for visitors to view documentation without
      needing to build it themselves.

In the `tok` project root, run

```sh
$ cargo doc --no-deps
```

The documentation will be found under `./target/doc/tok/index.html`
relative to the `tok` project root.

## Tips for Authors

### Writing YAML Files

Before writing content for your document, learn the
[YAML syntax](https://learnxinyminutes.com/docs/yaml/) (it's quite
simple).
The official website for YAML is [here](https://yaml.org/).

- [ ] TODO: provide a place for visitors to view documentation without
      needing to build it themselves.

To see which YAML keys are supported, build the documentation (see
above)

```sh
cargo doc --no-deps
```

The YAML keys that `tok` supports are documented in
`./target/doc/tok/yaml/struct.YamlData.html` and
`./target/doc/tok/node/struct.Node.html`.

Keys that are `String` types must contain valid LaTeX syntax in order
for the PDF to build.

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


## For Authors: Key Principles

The same way LaTeX is designed to free the author from worrying about
formatting a document and concentrate on content and organization, `tok`
is designed with the intent to free the author from _organization_ and
concentrate on content.

This is achieved by forcing the author to write about topics in
isolation, clearly, and independent of context.

The author only needs to keep track of dependency relationships between
topics.
The author does this by writing about a single topic in a YAML file that
holds information about the LaTeX environment in use (e.g. if the topic
is a definition or a theorem, `tok` will select the appropriate LaTeX
environment) with the `env` key.
The `after` (for predecessor paths) and `before` (for successor paths)
keys contain a sequence of other YAML files that must precede the
durrent topic or come after the topic in the final textbook.

By using `after` and `before`, there is no need to think about how to
organize a document for readers.
Also, depending on which file names `tok` receives as arguments, the
question of what to include and what to leave out for a given set of
topics is answered.

(For more information about supported YAML keys, see the section on
[Writing YAML Files]().)

Finally, because, each topic is (ideally) written independent of
context, `tok` allows a user (reader) to customize the organization of
the textbook according to their learning style using the `-r` option
without changing the dependency relationships defined in the YAML files'
`after` and `before` keys.

(For more information about supported command line options, see the
section on [Usage]().)





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
