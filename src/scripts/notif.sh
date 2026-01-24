#! /bin/zsh
set -e 

if ! command -v at 2>>/dev/null;then
    echo "you need install pakege at on system"
    exit 1 
fi
if sudo systemctl enable --now atd ;then
    echo 'notify-send "$1" "$2"'|at "$3" 
    exit 2 
fi 
