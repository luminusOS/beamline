#!/bin/sh
set -eu

desktop_file=$1
metainfo=$2
summary='Transmita sua tela para um monitor próximo'

grep -Fx "Comment[pt_BR]=$summary" "$desktop_file"
grep -F "<summary xml:lang=\"pt-BR\">$summary</summary>" "$metainfo"
