#!/usr/bin/env bash

set -eo pipefail

function generate() {
    n="$1"
    code=$(cat <<EOF
fn main(a_start: Field, b_start: Field) -> pub Field {
    let mut a = a_start;
    let mut b = b_start;
    for _ in 0..$n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}
EOF
)

    toml=$(cat <<EOF
[package]
name = "fib_$n"
type = "bin"
authors = [""]
compiler_version = "0.10.3"

[dependencies]
EOF
)

    mkdir -p "$n/src"
    echo "$code" > "$n/src/main.nr"
    echo "$toml" > "$n/Nargo.toml"
}

generate 1
generate 10
generate 100
generate 1000
generate 10000
generate 100000
generate 1000000
