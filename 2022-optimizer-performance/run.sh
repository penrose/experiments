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

# first build, to normalize later timings
cp dummy_objgrad.c src/objgrad.c
cp dummy_gen_code.rs src/gen_code.rs
cargo build --release

yarn

mkdir -p out/
mkdir -p results/
for subdir in $(basename penrose/packages/automator/out/* | grep -v '\.'); do
  ./grab.sh "$subdir"
  cargo build --release 2> out/"$subdir".txt
  target/release/penrose-experiment > out/"$subdir".json
  ./chart.js "$subdir" > results/"$subdir".svg
done
