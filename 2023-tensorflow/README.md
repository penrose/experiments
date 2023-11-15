# 2023 TensorFlow.js

author: [**@samestep**](http://github.com/samestep)

This experiment is to see how much slower Penrose is the autodiff engine is
replaced with [TensorFlow.js][].

## Results

All the data were gathered on my 2020 MacBook Pro with M1 chip. Each folder is
named according to the commit from the main [Penrose][] repo on which the code
was run.

For contrast, results from an experimental [Rose][] backend can be found in the
`rose` folder; hese were gathered on commit `4f177d5`. Also, results from a
JavaScript-emitting backend can be found in the `js` folder; these were gathered
on commit `23b4aaf`.

If you have Python installed, run `./analyze.py` to generate `data.csv`
comparing TensorFlow.js and Rose.

[Penrose]: https://github.com/penrose/penrose
[Rose]: https://github.com/rose-lang/rose
[TensorFlow.js]: https://www.tensorflow.org/js
