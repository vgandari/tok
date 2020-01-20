# tok

## Introduction

`tok` (stands for Tree of Knowledge) is a command line tool for
generating a textbook based on the learning objectives the user (reader)
has.

`tok` reads from YAML files that each store information for a topic of
interest, selects the indicated LaTeX environment indicated in the YAML
file, and organizes all topics before generating a TEX file based on
the dependency relationships expressed within the YAML files.
`tok` then compiles the TEX file into a PDF.

Each YAML file also contains information about its dependencies (other
YAML files) and which LaTeX environment to use.

The generated textbook is meant to be read in a purely sequential manner
(assuming the dependency relationships are defined correctly and
completely in all the YAML files).

### Motivation

- Textbook organiztion is not standardized or personalized. `tok` aims
  to solve both of these problems.
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
organize a textbook for students.
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

## Requirements

### Building

Rust and the `cargo` utility.
Install Rust and `cargo` from
[here](https://www.rust-lang.org/tools/install).

### Running

At the moment, `tok` is hardcoded to use `xelatex` to compile PDFs.

If you would like to use a different LaTeX engine, you can process the
generated TEX file on your own using that LaTeX engine.

The output of `tok` and `xelatex` will be under a directory called
`output` relative to where `tok` was run.

## Build

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

## Building the Documentation

In the `tok` project root, run

```sh
$ cargo doc --no-deps
```

The documentation will be found under `./target/doc/tok/index.html`
relative to the `tok` project root.

## Usage

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

The ouput will be `./output/main.pdf`.

You can customize the preamble, frontmatter, and backmatter in
`./texinput/preamble.tex`, `./texinput/frontmatter.tex`, and
`./texinput/backmatter.tex`, respectively.

You can also modify `./output/main.tex` directly and reprocess it using
the LaTeX engine of your choosing to generate a PDF if `xelatex` does
not produce the desired output.

### Writing YAML Files

Before writing content for your document, learn the
[YAML syntax](https://learnxinyminutes.com/docs/yaml/).
The official website for YAML is [here](https://yaml.org/).

#### YAML Keys

First, build the documentation (see above)

```sh
cargo doc --no-deps
```

The YAML keys that `tok` supports are documented in
`./target/doc/tok/yaml/struct.YamlData.html` and
`./target/doc/tok/node/struct.Node.html`.

Keys that are `String` types must contain valid LaTeX syntax in order
for the PDF to build.

### Why YAML?

Alternative formats considered were

- JSON
- TOML

JSON fomats do not allow multiline strings or comments.
This is unacceptable for users writing new text files to add to their
books.
YAML is also a superset of JSON.

TOML syntax requires quotes around strings, so YAML removes this extra
overhead.

### Conventions

Use `\eqref` for referencing equations.
This will provide a link to the equation in the generated PDF.

Name equations using `\label{eq:<path>__<name>}`, replacing `<path>`
with the path to the current file, and `<name>` with the name you would
like to use for this equation.

## As a Project Management Tool

The `unchecked` and `checked` types allow you to make to do list items
the same way you would create an item to include in your textbook.
The difference between `unchecked` and `checked` is that `unchecked`
produces and unchecked box and `checked` produces a checked box.
The boxes can be toggled on and off in the resulting PDF, although this
will not update the original YAML files.

The default sorting algorithm prioritizes items on the critical path.
With the `-r` option, the "lowest hanging fruit" comes first.

## Contributing

Feel free to open an issue or submit a pull request.
