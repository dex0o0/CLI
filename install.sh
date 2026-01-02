#!/bin/bash

cargo build --release
pwd=$(pwd)
cd $pwd/target/release || return
cp ./kali $pwd
