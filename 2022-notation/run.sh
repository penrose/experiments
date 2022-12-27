#!/usr/bin/env bash
set -eox pipefail

sha=05506951c8d6fedf117dae567c1b7e5535477d68
dir=penrose-$sha/packages/examples/src

wget -nc https://github.com/penrose/penrose/archive/$sha.zip
unzip -n $sha.zip
cargo build
find $dir \( -name '*.dsl' -o -name '*.sub' \) -exec ./example.sh $dir {} \;
