#!/usr/bin/env bash
set -eox pipefail

cd penrose/packages/automator/
yarn start batch registry.json ../../diagrams/ --src-prefix=../examples/src/
