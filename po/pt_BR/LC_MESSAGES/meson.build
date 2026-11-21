translation = custom_target('pt_BR-catalog',
  input: '../../pt_BR.po',
  output: 'beamline.mo',
  command: [msgfmt, '--check', '-o', '@OUTPUT@', '@INPUT@'],
  build_by_default: true,
  install: true,
  install_dir: get_option('localedir') / 'pt_BR/LC_MESSAGES',
)
