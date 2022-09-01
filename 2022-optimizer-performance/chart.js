#!/usr/bin/env node

import * as fs from 'fs';
import * as path from 'path';
import * as vega from 'vega';
import * as vegaLite from 'vega-lite';

if (process.argv.length !== 3) {
  throw Error(`expected 3 args, got ${process.argv.length}`);
}
const [, , name] = process.argv;

const slurp = (p) => fs.readFileSync(p).toString();
const slurpJSON = (p) => JSON.parse(slurp(p));

const meta = slurpJSON(path.join('penrose/packages/automator/out', name, 'meta.json'));
const optMs = meta.timeTaken.optimization;
const objGradMs = meta.timeTaken.justCallingObjGrad;

const data = slurpJSON(path.join('out', `${name}.json`));

const lines = slurp(path.join('out', `${name}.txt`)).split('\n');
const last = lines[lines.length - 2];
const compileSeconds = last.match(/(\d\.\d+)s/);
const compileMs = Number.parseFloat(compileSeconds[1]) * 1000;

const values = [
  {
    runtime: 'JavaScript',
    task: `b: gradient  ×${meta.numObjGradCalls}`,
    milliseconds: objGradMs,
  },
  {
    runtime: 'JavaScript',
    task: 'a: optimizer',
    milliseconds: optMs - objGradMs,
  },
  {
    runtime: 'native',
    task: `b: gradient ×${data.objgrad_count}`,
    milliseconds: data.objgrad_ms,
  },
  {
    runtime: 'native',
    task: 'a: optimizer',
    milliseconds: data.total_ms - data.objgrad_ms,
  },
];

let title = name;

if (compileMs > optMs) {
  title += ` (native compile time is ${compileSeconds[0]})`;
} else {
  values.push({ runtime: 'native', task: 'c: compile', milliseconds: compileMs });
}

const { spec } = vegaLite.compile({
  $schema: 'https://vega.github.io/schema/vega-lite/v5.json',
  data: { values },
  mark: 'bar',
  encoding: {
    y: {field: 'runtime', type: 'nominal'},
    x: {
      aggregate: 'sum',
      field: 'milliseconds',
      type: 'quantitative',
      title: 'milliseconds',
    },
    color: {field: 'task', type: 'nominal'},
  },
  title,
});

const view = new vega.View(vega.parse(spec));
console.log(await view.toSVG());
