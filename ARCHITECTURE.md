# Architecture

Beamline is at the bootstrap stage. Its documented foundation is Rust, GTK 4,
Libadwaita, Blueprint, Meson, Cargo, and gettext. Discovery and transmission
are not implemented, so this document does not describe a working data flow.

## Components and process boundaries

The GTK process owns the application window and its GLib main context. The
application ID is `org.luminusos.Beamline`. There is no daemon or D-Bus
control interface for the Shell yet.

## Repository layout

- `src/` contains the Rust entry point and Blueprint composite template.
- `data/` contains the resource manifest, desktop entry, AppStream metadata,
  and application icon.
- `po/` holds the Brazilian Portuguese catalog. English is the source language.
- `tests/` checks the strings read from the window in both languages.

## Build and startup

Meson compiles Blueprint to GtkBuilder XML, bundles it as a GResource, and
invokes Cargo. On startup, Beamline binds its gettext domain, registers the
embedded resource, and presents the window. Meson compiles and installs the
catalog and localizes the launcher metadata separately.

## Planned architecture

The future capture and transport code must stay outside the GTK window.
Screen capture will require consent through the desktop portal. An optional
Aurora Shell module may request an action through a versioned session D-Bus
interface; it will not own capture or receive video frames. Track that work
in [issue #5](https://github.com/luminusOS/beamline/issues/5).

The proposed interface name is `org.luminusos.Beamline.Cast1`. Start with a
target and session snapshot, updates, and explicit start/stop requests.
Discovery should be bounded and on demand. Treat bus callers as untrusted:
approve the destination in Beamline and obtain fresh portal consent rather
than reusing a stored capture token. Decide native and Flatpak activation in
a prototype before committing to a public D-Bus signature.

## Change impact

| Change | Review |
| --- | --- |
| Build or test workflow | Verify documented Meson commands against the project. |
| User-facing strings | Preserve gettext support and the initial `en` and `pt_BR` locales. |
| Application identity | Keep references to `org.luminusos.Beamline` consistent. |
| Discovery or transmission | Update architecture documentation only alongside verified implementation; distinguish planned work from available behavior. |
