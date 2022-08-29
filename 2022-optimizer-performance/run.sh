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

cp penrose/packages/automator/out/hypergraph-hypergraph/objgrad.c src/
cargo build --release
time target/release/penrose-experiment
