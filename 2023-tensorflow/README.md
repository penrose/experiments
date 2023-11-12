# 2023 TensorFlow.js

author: [**@samestep**](http://github.com/samestep)

This experiment is to see how much slower Penrose is the autodiff engine is
replaced with [TensorFlow.js][].

## Results

All the data were gathered on my 2020 MacBook Pro with M1 chip. Each folder is
named according to the commit from the main [Penrose][] repo on which the code
was run.

Four diagrams could not be run; they hung forever even with Vitest's `timeout`:

- `alloy-models/message-passing`
- `alloy-models/ring-leader-election`
- `alloy-models/workstations`
- `alloy-models/generic`

[Penrose]: https://github.com/penrose/penrose
[TensorFlow.js]: https://www.tensorflow.org/js
