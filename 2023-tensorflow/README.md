# 2023 TensorFlow.js

author: [**@samestep**](http://github.com/samestep)

This experiment is to see how much slower Penrose is the autodiff engine is
replaced with [TensorFlow.js][].

## Results

All the data were gathered on my 2020 MacBook Pro with M1 chip. Each folder is
named according to the commit from the main [Penrose][] repo on which the code
was run.

Three diagrams could not be run; they hung forever even with Vitest's `timeout`:

- `alloy-models/message-passing`
- `alloy-models/ring-leader-election`

For contrast, results from an experimental [Rose][] backend can be found in the
`rose` folder. These were gathered on commit `4f177d5`.

If you have [Node][] installed, run `csv.mjs` to generate `data.csv` comparing
TensorFlow.js and Rose.

[Node]: https://nodejs.org/en/download/
[Penrose]: https://github.com/penrose/penrose
[Rose]: https://github.com/rose-lang/rose
[TensorFlow.js]: https://www.tensorflow.org/js
