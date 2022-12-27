# 2022 Notation

author: [**@samestep**](http://github.com/samestep)

This experiment is an investigation into a possible way to bring back
`notation`. I wrote [nom][] parsers for a variation of our current Domain and
Substance languages.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

## Usage

First, make sure your working directory is the directory containing this README.
Then, run this command:

```sh
./run.sh
```

This will generate an `output/` folder containing copies of all the Domain and
Substance files from `packages/examples/src/` in the main Penrose repo, with the
output of this experimental parser appended in a comment at the bottom of each
file.

[nom]: https://github.com/Geal/nom
