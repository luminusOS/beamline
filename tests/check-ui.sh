#!/bin/sh
set -eu

binary=$1
locale_dir=$2

if ! command -v gtk4-broadwayd >/dev/null; then
    exit 77
fi

runtime_dir=$(mktemp -d)
pid=
cleanup() {
    if [ -n "$pid" ]; then
        kill "$pid" 2>/dev/null || :
        wait "$pid" 2>/dev/null || :
    fi
    rm -f "$runtime_dir/broadway98.socket"
    rmdir "$runtime_dir"
}
trap cleanup EXIT

export XDG_RUNTIME_DIR="$runtime_dir" GDK_BACKEND=broadway BROADWAY_DISPLAY=:97
export GTK_A11Y=none BEAMLINE_LOCALEDIR="$locale_dir"
gtk4-broadwayd :97 &
pid=$!
for iteration in 1 2 3 4 5 6 7 8 9 10; do
    if [ -S "$runtime_dir/broadway98.socket" ]; then
        break
    fi
    kill -0 "$pid"
    sleep 0.1
done
test -S "$runtime_dir/broadway98.socket"

english=$(LC_ALL=C.UTF-8 LANGUAGE=en "$binary" --check-ui)
printf '%s\n' "$english" | grep -Fx 'Cast Screen'
printf '%s\n' "$english" | grep -Fx 'Screen Casting Unavailable'
printf '%s\n' "$english" | grep -Fx 'Device discovery and screen casting are not available in this development build.'

portuguese=$(LC_ALL=en_US.UTF-8 LANGUAGE=pt_BR "$binary" --check-ui)
printf '%s\n' "$portuguese" | grep -Fx 'Transmitir tela'
printf '%s\n' "$portuguese" | grep -Fx 'Transmissão de tela indisponível'
printf '%s\n' "$portuguese" | grep -Fx 'A busca de dispositivos e a transmissão de tela não estão disponíveis nesta compilação de desenvolvimento.'
