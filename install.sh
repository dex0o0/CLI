#!/bin/bash

cargo build --release
pwd=$(pwd)
cd $pwd/target/release || return
cp ./kali $pwd

shell=$SHELL
if [ $shell == "/usr/bin/zsh" ];then
    echo "alias kali=$pwd/kali">>~/.zshrc
fi
if [ $shell == "/usr/bin/bash" ];then
    echo "alias kali=$pwd/kali">>~/.bashrc
fi
