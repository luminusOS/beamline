# Agent instructions

## Project scope

Beamline is an early-stage application. Its stack is Rust,
GTK 4, Libadwaita, Blueprint, Meson, Cargo, and gettext, with initial
translations in English (`en`) and Brazilian Portuguese (`pt_BR`).

Discovery and transmission are not implemented. Keep documentation and
contributions explicit about the difference between the current bootstrap and
future plans.

## Commands

The local commands are:

```sh
meson setup build
meson compile -C build
meson test -C build
meson compile -C build beamline-pot
```

Do not document additional commands, dependencies, test guarantees, or runtime
capabilities unless the project maintainer confirms them and the repository
supports them.

## Documentation changes

- Keep `README.md`, `CONTRIBUTING.md`, and `ARCHITECTURE.md` consistent.
- Link only to files and sections that exist in this repository.
- Describe future design as planned, not implemented.
- Keep the longer-term implementation plan in the LuminusOS workspace's
  `docs/beamline/` directory, outside this repository.

## Scope and repository hygiene

- Make only changes needed for the requested task.
- Do not add speculative scaffolding, workflows, or placeholder features.
- Use Conventional Commits when asked to commit. Do not commit or push
  without explicit authorization.
