#!/usr/bin/env bash

set -o errexit
set -o nounset

calc() {
    input="$@"

    echo "$@" | bc
}

fail_invalid_input() {
    echo "Error: invalid input"
    false
}

grains() {
    input=$1
    [[ $input == "total" ]] && calc "2^64 - 1" && return 
    [[ $input -gt 0 ]] || fail_invalid_input
    [[ $input -le 64 ]] || fail_invalid_input
    calc "2 ^ ($input - 1)"
}

grains "$@"
