#!/usr/bin/env bash
set -eox pipefail

pushd penrose/packages
yarn

pushd examples
yarn build
popd

pushd core
yarn build
popd

pushd automator
yarn start batch registry.json out/ --src-prefix=../examples/src/ --folders
popd

popd

diagram=hypergraph-hypergraph
thedir=penrose/packages/automator/out/$diagram

cp $thedir/objgrad.c src/
./gen_code.js $thedir > src/gen_code.rs

cargo build --release

time target/release/penrose-experiment
