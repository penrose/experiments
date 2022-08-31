#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

if (process.argv.length !== 3) {
  throw Error(`expected 3 args, got ${process.argv.length}`);
}
const [, , dir] = process.argv;
const get = stem => JSON.parse(fs.readFileSync(path.join(dir, `${stem}.json`)).toString());
const meta = get('meta');
const data = get('initial');

const lines = [
  'extern "C" {',
  '    fn objective_and_gradient(inputs: *const f64, gradient: *mut f64, secondary: *mut f64) -> f64;',
  '}',
  '',
  `pub const LEN_INPUTS: usize = ${data.varyingValues.length};`,
  'pub const LEN_GRADIENT: usize = LEN_INPUTS + 1;',
  `pub const LEN_OBJ_ENGS: usize = ${meta.numObjs};`,
  `pub const LEN_CONSTR_ENGS: usize = ${meta.numConstrs};`,
  'pub const LEN_SECONDARY: usize = LEN_OBJ_ENGS + LEN_CONSTR_ENGS;',
  '',
  'pub fn f(inputs: &[f64], gradient: &mut [f64], secondary: &mut [f64]) -> f64 {',
  '    assert_eq!(inputs.len(), LEN_GRADIENT);',
  '    assert_eq!(gradient.len(), LEN_GRADIENT);',
  '    assert_eq!(secondary.len(), LEN_SECONDARY);',
  '    unsafe {',
  '        objective_and_gradient(',
  '            inputs.as_ptr(),',
  '            gradient.as_mut_ptr(),',
  '            secondary.as_mut_ptr(),',
  '        )',
  '    }',
  '}',
  '',
  '#[allow(dead_code)]',
  '#[derive(PartialEq)]',
  'pub enum InputMeta {',
  '    Sampler,',
  '    Pending,',
  '}',
  '',
  'pub const INPUTS: [InputMeta; LEN_INPUTS] = [',
  ...data.inputs.map(v => `    InputMeta::${{ sampler: 'Sampler', pending: 'Pending' }[v]},`),
  '];',
  '',
  'pub const VARYING_VALUES: [f64; LEN_INPUTS] = [',
  ...data.varyingValues.map(v => `    ${v}f64,`),
  '];',
];

console.log(lines.join('\n'));
