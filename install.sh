#!/bin/bash

cargo build --release
pwd=$(pwd)
cd $pwd/target/release
cp ./kali $pwd
