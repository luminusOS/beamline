project('beamline', version: '0.1.0', meson_version: '>=1.3')

app_id = 'org.luminusos.Beamline'
dependency('gtk4', version: '>=4.24')
dependency('libadwaita-1', version: '>=1.10')

blueprint = find_program('blueprint-compiler', version: '>=0.22')
resource_compiler = find_program('glib-compile-resources')
cargo = find_program('cargo')
shell = find_program('sh')

window_ui = custom_target('window-ui',
  input: 'src/window.blp',
  output: 'window.ui',
  command: [blueprint, 'compile', '--output', '@OUTPUT@', '@INPUT@'],
)

resource = custom_target('beamline-gresource',
  input: ['data/beamline.gresource.xml', window_ui],
  output: 'beamline.gresource',
  command: [resource_compiler, '--sourcedir', meson.current_build_dir(),
            '--target', '@OUTPUT@', '@INPUT0@'],
)

beamline = custom_target('beamline',
  output: 'beamline',
  input: resource,
  depend_files: files('Cargo.toml', 'Cargo.lock', 'src/main.rs', 'src/window.rs'),
  command: [shell, files('build-aux/cargo-build.sh'),
            cargo, meson.project_source_root(), meson.current_build_dir(),
            get_option('prefix') / get_option('localedir'), resource.full_path(),
            '@OUTPUT@', get_option('buildtype')],
  install: true,
  install_dir: get_option('bindir'),
)

subdir('po')
subdir('data')

test('localized-desktop-metadata', shell,
  args: [files('tests/check-metadata.sh'), desktop_entry, metainfo],
)

test('localized-window', shell,
  args: [files('tests/check-ui.sh'), beamline.full_path(),
         meson.current_build_dir() / 'po'],
  depends: [beamline, translation],
)
