# tok

## Introduction

`tok` (stands for Tree of Knowledge) is a command line tool for
generating a textbook based on the learning objectives the user (reader)
has.
The input is a YAML file or set of YAML files where each file contains
information about a topic of interest.
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

Finally, because, each topic is (ideally) written independent of
context, `tok` allows a user (reader) to customize the organization of
the textbook according to their learning style using the `-r` option
without changing the dependency relationships defined in the YAML files'
`after` and `before` keys.

## Requirements

### Building

Rust and the `cargo` utility.
Install Rust and `cargo` from
[here](https://www.rust-lang.org/tools/install).

### Running

At the moment, `tok` is hardcoded to use `xelatex` to compile PDFs.
You can choose to use whichever LaTeX you want on the generated TEX file
instead.

## Build

From the command line a the root of the project, run

```sh
$ cargo build --release
```

## Install

If you would like to install `tok` to your PATH, run

```sh
$ cargo install --path .
```

By default, `cargo` will install `tok` in `$HOME/.cargo/bin`.
Make sure this is in your PATH.
If you configured `cargo` to install binaries somewhere else, make sure
that directory is in your PATH.

## Usage

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

After the program finishes running, there will be a file called
`zzz.pdf`.

If you would like to explore/edit the book directly, you may do so by
editing `zzz.tex` and generating a new PDF from LaTeX (using the LaTeX
engine of your choice).

### Command Line Options

You can view the various options by

```sh
$ tok --help
```

Note that some options may be labeled `[INOP]`.
That means that these options are inoperable and have no effect in the
current version.

## For Authors: Writing YAML Files

`tok` uses YAML files to represent topics of interest and to extract the
necessary information to properly format each topic within the book.
The YAML files also express dependency relationships between each other,
so even providing only one YAML file may produce a very large volume,
depending on the dependency relationship between the topic expressed in
that YAML file and other topics (i.e. how advanced the topic is).
It also depends, of course, on how many YAML files are already written
and available to `tok`.

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

### YAML Keys

- [ ] Provide guide for all keys, make code reflect this section

The `pre`, `main`, and `post` keys should be multiline strings (unless
you have very little to say about a given topic!):

```yaml
pre: |
  Verbatim text
  that spans multiple lines
  and copies special "characters"
  verbatim\\\ <-not escaped characters!
```

Note the pipe `|` and indented text.

The `main` key in particular uses the LaTeX environment defined in the
`env` key.
Make sure this environment is defined in your preamble if not using the
default preamble.

> Note: The verbatim multiline strings are meant to store
> LaTeX-formatted text. They will be copied into the generated TEX file.
> That is, for `pre`, `main`, and `post`, write as if you are writing
> directly into the resulting TEX file!

### Conventions

Use `\eqref` for referencing equations.
This will provide a link to the equation in the generated PDF.

Name equations using `\label{eq:<path>__<name>}`, replacing `<path>`
with the path to the current file, and `<name>` with the name you would
like to use for this equation.

## As a To Do List Generator

The `unchecked` and `checked` types allow you to make to do list items
the same way you would create an item to include in your textbook.
The difference between `unchecked` and `checked` is that `unchecked` produces
and unchecked box and `checked` produces a checked box.
The boxes can be toggled on and off in the resulting PDF.

For generating task lists, the `-r` option is recommended.

## For Developers (and the curious): How it Works

- [ ] Write this section
