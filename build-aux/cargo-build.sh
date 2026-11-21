#!/bin/sh
set -eu

cargo=$1
srcdir=$2
builddir=$3
export BEAMLINE_LOCALEDIR=$4
export BEAMLINE_GRESOURCE=$5
export CARGO_TARGET_DIR="$builddir/cargo"

output=$6
case "$7" in
    release | minsize) set -- --release; profile=release ;;
    debugoptimized)
        export CARGO_PROFILE_RELEASE_DEBUG=true
        set -- --release
        profile=release
        ;;
    *) set --; profile=debug ;;
esac

"$cargo" build --locked --manifest-path "$srcdir/Cargo.toml" "$@"
cp "$CARGO_TARGET_DIR/$profile/beamline" "$output"
