catalog_sources = files('../po/LINGUAS', '../po/pt_BR.po')
catalog_dir = meson.project_source_root() / 'po'

desktop_entry = custom_target('desktop-entry',
  input: app_id + '.desktop.in',
  output: app_id + '.desktop',
  command: [msgfmt, '--desktop', '--template=@INPUT@', '-d', catalog_dir, '-o', '@OUTPUT@'],
  depend_files: catalog_sources,
  install: true,
  install_dir: get_option('datadir') / 'applications',
)

metainfo = custom_target('metainfo',
  input: app_id + '.metainfo.xml.in',
  output: app_id + '.metainfo.xml',
  command: [msgfmt, '--xml', '--template=@INPUT@', '-d', catalog_dir, '-o', '@OUTPUT@'],
  depend_files: catalog_sources,
  install: true,
  install_dir: get_option('datadir') / 'metainfo',
)

install_data(app_id + '.svg',
  install_dir: get_option('datadir') / 'icons/hicolor/scalable/apps',
)
