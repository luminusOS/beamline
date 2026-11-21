msgfmt = find_program('msgfmt')
xgettext = find_program('xgettext')
subdir('pt_BR/LC_MESSAGES')

run_target('beamline-pot',
  command: [shell, files('../build-aux/update-pot.sh'), xgettext,
            meson.project_source_root(), meson.project_build_root()],
  depends: window_ui,
)
