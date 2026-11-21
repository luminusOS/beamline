<img src="data/org.luminusos.Beamline.svg" alt="" width="96">

# Beamline

Beamline is a wireless screen casting application in development for GNOME.
Its interface is written in Rust with GTK 4, Libadwaita, and Blueprint.

> [!NOTE]
> The current build opens the application window in English or Brazilian
> Portuguese. It does not discover devices or transmit a screen yet.

## Getting started

Use the GNOME 51 SDK or a development environment with GTK 4.24, Libadwaita
1.10, Blueprint Compiler 0.22, Rust 1.92 or later, Meson, Ninja, and GNU gettext.

```sh
meson setup build
meson compile -C build
meson test -C build
./build/beamline
```

The last command opens the development build in your graphical session. To
try Brazilian Portuguese before installing, run the following with the
`en_US.UTF-8` locale available:

```sh
LC_ALL=en_US.UTF-8 LANGUAGE=pt_BR BEAMLINE_LOCALEDIR="$PWD/build/po" ./build/beamline
```

The application ID is `org.luminusos.Beamline`.

## Roadmap

Miracast is the first target. Google Cast and browser-based casting via WebRTC
are planned separately. Aurora Shell integration will be optional; Beamline
will remain a standalone application.

## Contributing

See the [contributor guide](CONTRIBUTING.md), the
[architecture notes](ARCHITECTURE.md), and the
[issue tracker](https://github.com/luminusOS/beamline/issues).
