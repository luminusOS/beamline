# Contributing to Beamline

Keep contributions focused, explain the user-visible or maintenance problem,
and include evidence that reviewers can reproduce.

## Start with an issue and branch

Check existing issues and discuss broad behavior or architecture changes with
the maintainers before implementation. The repository is in its bootstrap
stage: discovery and transmission are not implemented. Do not present planned
behavior as working functionality.

Read [ARCHITECTURE.md](ARCHITECTURE.md) before changing project boundaries or
introducing architecture that affects future work.

## Choose validation

The Meson workflow is:

```sh
meson setup build
meson compile -C build
meson test -C build
```

To generate the gettext template:

```sh
meson compile -C build beamline-pot
```

Report the exact commands and outcomes you ran. Do not claim tests, runtime
checks, or capabilities that you did not verify.

## Translations

English is the source language. Mark Blueprint strings with `_()` and
update `po/pt_BR.po` from the generated template:

```sh
meson compile -C build beamline-pot
msgmerge --update --backup=none po/pt_BR.po build/po/beamline.pot
msgfmt --check -o /dev/null po/pt_BR.po
meson test -C build
```

The localized window test uses `gtk4-broadwayd` and the `en_US.UTF-8` locale
available in the GNOME 51 SDK. Meson reports it as skipped when Broadway is
unavailable.

## Commits

Use Conventional Commits with an imperative subject in English. Explain why
the change is needed when the subject is not enough.

## Pull requests

Keep each change focused. A pull request should explain:

1. What problem it addresses.
2. Why the change belongs in the affected component.
3. What risks or limitations remain.
4. What commands or other evidence reviewers can use to verify it.

For behavior changes, include clear reproduction steps. For documentation-only
changes, check that internal links and command examples match the repository.

## AI-assisted contributions

Contributors remain responsible for every submitted change, including
AI-assisted work. Review generated material for unsupported features,
nonexistent commands, and claims that exceed the implementation.
