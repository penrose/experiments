#!/usr/bin/env bash
out=output${2#"$1"}
mkdir -p "$(dirname "$out")"
cp "$2" "$out"
{
  echo
  echo
  echo '/*'
  target/debug/penrose-experiment "$2"
  echo '*/'
} >> "$out"
