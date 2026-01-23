#!/bin/zsh
set -e 
echo "Building dex"
if ! cargo build --release; then 
  echo "failed to Building"
  exit 1
else
  echo "..Building completed.."
fi
current_dir=$(pwd)
shell_name=$(basename $SHELL)
shell_rc="$HOME/.${shell_name}rc"
binary_path="$current_dir/target/release/dex"
if [ ! -f "$binary_path" ]; then 
  echo "Binary not found:$binary_path"
  exit 1
fi
if [ ! "$shell_rc" ];then
  touch "$shell_rc"
fi
echo "check file:$shell_rc"
if ! grep -q "alias dex='exec $binary_path'" "$shell_rc";then
  echo "alias dex='exec $binary_path'">>"$shell_rc"
  echo "added"
else
  echo "alredy exists config in $shell_rc"
fi
chmod +x "$binary_path"
source "$shell_rc"
echo "........completed......."
