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
yarn start draw hypergraph.sub hypergraph.sty hypergraph.dsl out --src-prefix=../examples/src/hypergraph --variation=ConchSpoonbill0283
popd

popd

cp penrose/packages/automator/out/objgrad.c src/
cargo build --release
time target/release/penrose-experiment
