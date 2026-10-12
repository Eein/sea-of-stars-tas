#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
output_dir="$repo_root/target/crossover-joystick"
compiler=${MINGW_CC:-x86_64-w64-mingw32-gcc}

if ! command -v "$compiler" >/dev/null 2>&1; then
    printf '%s\n' "Missing $compiler. Run: nix develop -c sh xinput/build.sh" >&2
    exit 1
fi

mkdir -p "$output_dir"
"$compiler" -std=c11 -O2 -Wall -Wextra -Werror -shared \
    "$repo_root/xinput/xinput1_3.c" "$repo_root/xinput/xinput1_3.def" \
    -o "$output_dir/xinput1_3.dll"
printf '%s\n' "$output_dir/xinput1_3.dll"
