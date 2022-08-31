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

mkdir -p out/
for subdir in $(basename penrose/packages/automator/out/* | grep -v '\.'); do
  ./grab.sh "$subdir"
  cargo build --release
  target/release/penrose-experiment > out/"$subdir".json
done
