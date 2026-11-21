#!/bin/sh
set -eu

xgettext=$1
srcdir=$2
builddir=$3
pot="$builddir/po/beamline.pot"

cd "$srcdir"
"$xgettext" --from-code=UTF-8 --package-name=Beamline --language=Desktop \
    -o "$pot" data/org.luminusos.Beamline.desktop.in
"$xgettext" --from-code=UTF-8 --join-existing --language=Glade \
    -o "$pot" "$builddir/window.ui"
"$xgettext" --from-code=UTF-8 --join-existing --language=Appdata \
    -o "$pot" data/org.luminusos.Beamline.metainfo.xml.in
