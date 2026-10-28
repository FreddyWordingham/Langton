#!/usr/bin/env bash

set -euo pipefail
shopt -s nullglob
export LC_ALL=C

for tool in rsvg-convert magick; do
    command -v "$tool" >/dev/null || {
        echo "Missing command: $tool" >&2
        exit 1
    }
done

[[ -d ./resources/tiles ]] || {
    echo "Missing resources/tiles directory" >&2
    exit 1
}

for dir in ./resources/tiles/*/; do
    group=${dir%/}
    group=${group##*/}
    sources=("$dir"*.svg)
    ((${#sources[@]} > 0)) || continue

    destination="./assets/sprites/$group"
    mkdir -p "$destination"
    sprites=()

    for svg in "${sources[@]}"; do
        filename=${svg##*/}
        png="$destination/${filename%.svg}.png"

        # Preserve proportions and center on a transparent 64×64 canvas.
        rsvg-convert --width=64 --height=64 --keep-aspect-ratio "$svg" |
            magick png:- -background none -gravity center -extent 64x64 "$png"

        sprites+=("$png")
    done
done
