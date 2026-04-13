#!/bin/bash

CURRENT_DIR=$(pwd)
ROOT_BIN="/usr/bin/"
SHELL_NAME=$(basename $SHELL)
SHELL_RC="$HOME/.${shell_name}rc"
BINARY_PATH="$CURRENT_DIR/target/release/dex"

check_cargo(){
  echo "building project"
  if cargo build --release &>/dev/null;then
    echo "build success"
    return 0
  else
    echo "build Failed"
    return 1
  fi
  
}

error_exit(){
  echo $1
  exit 1 
}

if check_cargo;then
  echo "create \"dex\" binary" && sudo mv $BINARY_PATH $ROOT_BIN || error_exit "Error to moving $BINARY_PATH to $ROOT_BIN"
  echo "<----------------completed------------------->"
else
  echo "please check cargo installed"
fi
