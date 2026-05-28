#!/usr/bin/env bash
# Bundle a CSES bin file with src/lib.rs into one uploadable file.
# Writes to dist/<bin_name>.rs and prints the path.
# Usage: tools/bundle.sh <bin_name>
# Example: tools/bundle.sh 1068_weird_algorithm

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "usage: $0 <bin_name>" >&2
    exit 1
fi

bin_file="src/bin/$1.rs"
if [ ! -f "$bin_file" ]; then
    echo "not found: $bin_file" >&2
    exit 1
fi

mkdir -p dist
out_file="dist/$1.rs"

{
    # Lib source with `pub ` stripped so items are private to the single file.
    sed 's/^pub //; s/ pub / /g' src/lib.rs
    echo
    # Bin source with `cses::` namespace removed.
    sed 's/cses:://g' "$bin_file"
} > "$out_file"

echo "$out_file"
