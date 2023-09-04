#!/bin/sh

set -e

if [ $# -ne 1 ]; then
  exit 1
fi

cargo build --release
sudo flamegraph -F 99 -- target/release/$1
