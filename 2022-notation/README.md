# 2022 Notation

author: [**@samestep**](http://github.com/samestep)

This experiment is an investigation into a possible way to bring back
`notation`. I wrote [nom][] parsers for a variation of our current Domain and
Substance languages. The idea was: we are currently using [nearley][] for
parsing, so because it exposes its parser generator at compile time instead of
at runtime, we are unable to use it to generate a parser from a set of custom
notations provided by a Penrose user in a Domain program. Perhaps we could find
a more flexible parser library which would allow us to generate the Substance
parser dynamically.

## Results

It wasn't very hard to write these small parsers. However, after trying this, I
don't think I would want to use nom to write a parser for a programming
language. Its [`alt`][] combinator is local and greedy, not really comparable to
the global conflict/ambiguity analysis you get from tools like [LALRPOP][] or
[grmtools][], or the ability to parse ambiguous grammars like nearley can. It
seems like probably the best solution would be to keep our Domain and Style
parsers as-is, and reuse our existing [Moo][] lexer for Substance, then
handwrite a parametrizable Substance parser to support custom `notation`.

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

[`alt`]: https://docs.rs/nom/7.1.1/nom/branch/fn.alt.html
[grmtools]: https://github.com/softdevteam/grmtools
[lalrpop]: https://github.com/lalrpop/lalrpop
[moo]: https://github.com/no-context/moo
[nearley]: https://nearley.js.org/
[nom]: https://github.com/Geal/nom
