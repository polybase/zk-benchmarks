#!/usr/bin/env bash

set -eo pipefail

field_bits=254

function generate() {
    n_bytes="$1"
    n_fields="$(python3 -c "import math; print(math.ceil($n_bytes / math.floor($field_bits / 8)))")"
    n="$n_fields"

    code="
use dep::std;

fn main(x: [Field; $n]) -> pub [Field; 2] {
    std::hash::pedersen(x)
}"
    toml="
[package]
name = "pedersen_$n"
type = "bin"
authors = [""]
compiler_version = "0.10.3"

[dependencies]
"

    mkdir -p "$n/src"
    echo "$code" > "$n/src/main.nr"
    echo "$toml" > "$n/Nargo.toml"
}

generate 1000
generate 10000
generate 100000
