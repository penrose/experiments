# 2022 Autodiff Rewrite Determinism

author: [**@samestep**](http://github.com/samestep)

This experiment is a set of debugging tools I wrote in early 2022 while
[rewriting the symbolic differentiation engine][rewrite]. The goal was to
produce exactly the same results as the previous symbolic differentiation
engine, but for a while many of the generated diagrams were slightly different.
In the end I realized that this was due to two things:

1. [I was adding up the partial derivatives in a different order.][order]
2. [I wasn't special-casing the 1- and 2-argument cases for n-ary ops.][nary]

TODO

[nary]: https://github.com/penrose/penrose/pull/907/commits/12675945ce217116c2310410a8d04d0a1e9a6ab9
[order]: https://github.com/penrose/penrose/pull/907/commits/9a31168843f41591f178384595da0ff6e38e4d6a
[rewrite]: https://github.com/penrose/penrose/pull/907
