#!/bin/zsh
set -e
if command -v at;then
    echo 'notify-send "$1" "$2"'| at "$3"
fi
