#!/usr/bin/env bash
set -eox pipefail

thedir=penrose/packages/automator/out/"$1"

cp $thedir/objgrad.c src/
./gen_code.js $thedir > src/gen_code.rs
