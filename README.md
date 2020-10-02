# Tree of Knowledge

## Introduction

Tree of Knowledge `tok` is a knowledge and project management tool that

1. takes your notes and organizes them so that they are easy to follow.
2. takes your tasks, assigns a priority, and generates a to do list, or
   project plan.

For the first case, think of `tok` as a texbook generator that lifts the
burden of figuring out where to put each paragraph.
For the second case, ask, "What should I work on first/right now," and
`tok` will give you the answer.

Let's say you have a bunch of files, each containing notes covering a
particular topic.

- How should you arrange these topics so that they are easy to follow?
- Which topics should you include?

Equivalently,

- How should I sort my tasks so that I start only what I can start?
- Which subtasks are relevant to a task/Which tasks are relevant to a
  project?

The answers to these questions depend on how the topics/tasks depend on
each other.
Each topic/task is defined in a YAML file.
Each file contains text for a topic, some formatting metadata, and
dependency relationships (see an example [below](#example-yaml-file)).
`tok` uses dependency relationships to determine the "best" order to
place your notes.
Once `tok` computes the "best" order to sort the topics, it writes your
notes in a TEX file and generates a PDF.

The "best" order to sort topics always respects dependency relationships
except where dependency relationships form a cycle.
When using `tok` to manage a project, the sorting is modified to push
tasks with deadlines towards the beginning of the final document.
That is, tasks with a deadline always take priority over tasks without a
deadline.
This is explained more in [Tasks and Deadlines](#tasks-and-deadlines).

`tok` generates a document based on the input files and whatever other
files they depend on.
This means that `tok` may generate a document that contains a subset of
your notes/tasks.
That is, `tok` leaves out the irrelevant topics/tasks and sticks to the
focus of the document you are generating.

### Goals

- **Organize notes automatically** so that they may serve as a
  **comprehensive reference without unnecessary information**.
- Provide readers with a way to **customize organization of a document
  while respecting the author-specified dependency relationships between
  topics**.
- **Unburden readers from the task of revisiting information** presented
  earlier/later in a text in order to understand information presented
  later/earlier; i.e., produce a **document designed to be read and
  studied sequentially**.
- Allow **authors to focus on individual topics in isolation** and
  remove the burden of finding the optimal order in which to present
  topics to readers.
- Provide readers with a way to **strip away unnecessary information**
  leading up to a topic of interest while presenting all necessary
  information.
  By stripping away unneccesary information and maintaining an optimal
  ordering of topics, the **reader no longer has to "hunt" for relevant
  information**.
- **Generate a to do list that scales to a large project**, based on
  deadlines (if applicable) and dependencies between tasks.

I use `tok` as:

- My research/lab notebook
- Task/project manager
- A textbook for everything I've learned from classes, papers, etc. that
  I can reference or share with students. See
  ["Everything...Or at Least Some of It"](https://github.com/vgandari/everything).

## Install

`tok` requires Rust and the `cargo` utility.
Install Rust and `cargo` from
[here](https://www.rust-lang.org/tools/install).
To generate PDFs, `tok` also requires
[LaTeX](https://www.latex-project.org/get/).

If you would like to install `tok` to your PATH, run

```sh
cargo install --path .
```

from the `tok` project root.
By default, `cargo` will install `tok` in `$HOME/.cargo/bin`.
Make sure this is in your PATH.
If you configured `cargo` to install binaries somewhere else, make sure
that directory is in your PATH.

To update, navigate to the directory where you cloned `tok` and run

```sh
git pull
cargo install --path . --force
```

## Usage

See `tok --help` for command line options.
Options marked `[INOP]` are inoperable and have no effect.

By default, `tok` uses `xelatex` to compile PDFs.
You can use a different engine to compile the generated TEX file by
specifying `--engine=pdflatex` or `--engine=luatex`.
You can also generate a TEX file without compiling to PDF using the
`--no-pdf` option.
You can always generate the PDF with whatever LaTeX engine you want once
the TEX file is generated.

To include _all_ files in your document's project directory (*nix
systems), run

```sh
tok $(find . -name '*.yaml' -print)
```

### Structuring a Project for `tok`

```
project/
|_ code/               <-- for code listings
|_ images/             <-- for figures
|_ output/             <-- all output will go here;
|                          see .gitignore
|_ texinput/
|  |_ backmatter.tex   <-- optional
|  |_ frontmatter.tex  <-- optional
|  |_ preamble.tex     <-- optional
|_ main/               <-- location of yaml files;
                           can be any name;
                           also where the `tok` command is run
```

If using Git as your VCS/SCM, include the following in your
`.gitignore`:

```
!code/
!images/
output/
!texinput/preamble.tex
!texinput/frontmatter.tex
!texinput/backmatter.tex
```

The output of `tok` and LaTeX will be under a directory called
`../output` relative to where `tok` was run (e.g. if running `tok` in
`project/main/` above, the output files will be located in
`project/output/`).

Each file should be named with the assumption that it will appear as a
chapter or section heading, even if the prefix is `x`.
This is because the `--headings` and `--extra-headings` options will
make `tok` analyze the structure of the document and insert chapter and
section headings where appropriate; the user does not decide where the
headings are inserted.
More information about headings can be found under [Heading
Generation](#heading-generation).

### Example YAML File

[Here's a nice guide for learning the YAML syntax.](https://learnxinyminutes.com/docs/yaml/)
Below is an example of a YAML file suitable for a project made for
`tok`.
The values that `tok` recognizes are explained more generally in the
next sections.
Note the pipes `|` where multiline strings are required.

```yaml
aka:
  - dog
  - canine
  - good boy
eli5: |
  To avoid overwhelming your audience, you can "Explain like I'm Five".
  This is a special section where you can provide possibly
  oversimplified explanations of the current concept.
  It's great for an introductory text, a rough draft, or an idea that
  came to you in the middle of the night that you simple have to write
  down right now.
pre: |
  Here's some text that will appear in the document, introducing the
  topic stored in this file.
  Maybe we want to motivate a definition, or provide some general
  context for the main content.
  Put whatever you think is appropriate before introducing the main
  text.
main: |
  Here is where we write the main text.
  If the file name has a prefix `def`, `thm`, etc., then the text here
  will appear as a definition, theorem, etc. in the final document.
  All text inside of `pre` and `post` will appear as plain text in the
  document body.
post: |
  Now that we've presented the main idea, we can discuss it a little
  further.
  Maybe you want to clarify something that people often get confused.
  Maybe you can discuss how the theorem you've just presented is
  applied.
# Here we tell tok which files to include that must appear earlier in
# the document.
req:
  - ./required_file_1.yaml
  - ./required_file_2.yaml

# Some topics aren't required for the reader to understand the current
# topic, but you may feel the need to include these topics anyway for
# completeness. This is where you should tell tok to include these
# files.
# They will appear later in the final document.
# If there are any files included in the document that are not required
# for any of the files passed to tok in the command line, an appendix
# will be generated.
incl:
  - ./some_other_file_1.yaml
  - ./some_other_file_2.yaml

# URLS are key-value pairs.
# They key is the text you want to display in text, and the value is the
# URL for the link.
urls:
  google: https://google.com
  ? |
    long_name
  : |
    https://long_url.com

# Any questions you need to answer.
# If you're taking notes in class, these could be questions you will
# look up later after lecture.
q:
  - Why does the Earth go around the Sun?
  - Find original source for this node.

# BibTeX style references.
# You can use the same references across different files.
# tok will gather references from all the relevant YAML files, generate
# a BIB file, and automatically remove duplicates.
# You can cite these references from anywhere in the resulting TEX file.
# Beause these sources can be cited from anywhere in the TEX file,
# it's possible to cite sources declared in a different YAML file.
# Because it's hard to keep track of what other YAML files will
# ultimately be included in the final document, it is recommended to
# include the sources you're going to cite in the same YAML file where
# you cite them.
# Since tok removes duplicate entries, it's better to err on the side of
# declaring too many sources in a single file and declaring duplicates
# across many files than to attempt to keep track of which sources are
# included in other files.
src:
  - |
    @article{src1
      ...
    }
  - |
    @article{src2
      ...
    }

# For files with prefixes thm, lem, cor, rem, you can include proofs and
# tok will include them in a proof environment.
# You can include as many proofs as you like
pfs:
  - |
    A proof
  - |
    Another, more elegant proof
  - |
    Yet another proof taking a different approach

# Indicate that there is no Wikipedia page for this topic -- you've
# checked.
# Files with a prefix `x`, `task`, and `done`, are treated as if this
# option is automatically set to true.
nowiki: true

# The name of this topic as it appears in the final document is on
# Wikipedia under a different name, or in a section of a Wikipedia
# article.
# Links like this will appear as "Name" on Wikipedia, instead of Search
# for "Name" on Wikipedia to indicate that the page definitely exists.
wiki: https://wikipedia.org/name_different_from_file_name#or_name_of_section

# The folloing are ignored for environments that are not task
# Dates are stored [YYYY, MM, DD] and are shown here using JSON syntax,
# since YAML is a superset of JSON.
deadline: [1642, 12, 25]
start: [1642, 12, 25]
complete: [1642, 12, 25]
# If this node represents a task, we expect it to take about a week, so
# we put 7, indicating that this task should take seven days to
# complete.
expected: 7
```

### Defining a Dependency Graph

Each YAML file in your project represents a node.
`tok` reads these YAML files and constructs a graph based on the
dependency relationships between the nodes.
Dependency relationships between nodes are declared within YAML files.
The following keys are for expressing dependency relationships:

 - `req`: Text stored in this node must appear "after" all text
   contained in the files in this list. All nodes listed under "req" are
   required for understanding material in this node's text.
 - `incl`: Text stored in this node must appear "before" all text
   contained in the files in this list. Nodes listed under "incl" are
   not required for understanding material in this node's text, but are
   nodes the author has decided to include anyway. These nodes do not
   appear if the option `--sdepth=0` is passed to the command line. The
   size of the document grows with increasing value for `sdepth`.

For example, if node A depends on node B (i.e. text stored in A must
appear later in the document than text stored in B), then you may write

```yaml
req:
  - B.yaml
```

inside of `A.yaml`, and/or

```yaml
incl:
  - A.yaml
```

inside of `B.yaml`.

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

- topics and tasks
  - `aka`: "Also known as", for including alternate labels in document
  - `pre`: Text for introducing the main text, outside of any LaTeX
    environment.
  - `main`: The main text that appears inside your environment
  - `post`: Text for providing more discussion after the main text,
    outside of any LaTeX environment.
  - `urls`: Any URLs that might be helpful
  - `eli5`: "Explain like I'm five", a simple explanation, even if not
    technically correct.
  - `src`: BibTeX items; `tok` will automatically generate a BIB file
    free of duplicates
- topics
  - `pfs`: If using the `thm`, `lem`, `cor`, or `rem` environments,
    include one or more proofs
- tasks
  - `deadline`: deadline of a task, e.g. `[1642, 12, 25]`
  - `expected`: expected duration of a task, in days
  - `start`: start date of a task, e.g. `[1642, 12, 25]`
  - `complete`: completion date of a task, e.g. `[1642, 12, 25]`
  - `assgn`: list of names of people to whom a task is assigned

> NOTE: `pre`, `main`, `post`, `pfs`, and `eli5` keys must contain valid
> LaTeX code.

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
- `x` - plain text, hide title, set `nowiki: true`; actually, you can
  use any prefix that `tok` does not have rules for and use a custom
  environment defined in your preamble for your `main` text.
- `task` - plain text, show title in bold before any text from this
  node, show "TASK" with empty checkbox in left margin
- `done` - plain text, show title in bold before any text from this
  node, show "DONE" with an "X" in left margin

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

### Tasks and Deadlines

Tasks can be assigned deadlines, and those deadlines will affect the
order in which node content appears in the final document.
All nodes that use the `task` "environment" and also have a deadline
defined will appear before nodes that do not have deadlines.
The only exception is if nodes with deadlines have dependencies that are
not tasks or do not have deadlines.

Tasks can also be assigned expected duration, which affects which
branches containing a series of nodes appear first in the document.
In order to keep track of performance, tasks also have start dates and
completion dates.
The actual duration of a task is computed from thedifference between the
start and completion dates.

<!-- You can export the times (expected and actual durations, start dates, -->
<!-- deadlines, completion dates) for a person responsible for tastks or for -->
<!-- a project more broadly to perform analysis of -->
<!-- individual/team/organizatio performance, etc. -->

<!-- Because `tok` is designed to allow users to focus on documenting -->
<!-- individual nodes without needing to take into account everything in a -->
<!-- textbook or project at once, you may accidentally set a deadline for a -->
<!-- task that is earlier than a deadline for one or more of its subtasks. -->

If there is a dependency relationship between two tasks, `tok` will not
break that relationship, even if it results in a task with a later
deadline appearing before a task with an earlier deadline.
In that scenario, the task with the earlier deadline (the one that
appears later) will show its deadline in red, indicating that something
might be wrong with this project's schedule -- a task should not have an
earlier deadline than the task(s) it depends on.

### Heading Generation

The `--headings` and `--extra-headings` options generate and insert
headings into the generated document to break it up into sections.
Depending on what `tok` determines to be the maximum heading depth,
headings will be generated based on the table below.

Max Depth | Heading Types
-- | --
0 | No headings
1 | Sections
2 | Sections, Subsections
3 | Chapters, Sections, Subsections
4 | Chapters, Sections, Subsections, Subsubsections
5 | Parts, Chapters, Sections, Subsections, Subsubsections
6 | Books/Volumes, Parts, Chapters, Sections, Subsections, Subsubsections

The maximum heading depth computed is higher with the `--extra-headings`
option than with the `--headings` option.
For short documents, `--headings` has no effect.
Even for longer documents, `--headings` is recommended for generating a
draft that is closer to the final version of the document.
