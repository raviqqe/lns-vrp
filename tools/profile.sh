#!/bin/sh

set -e

cargo build --release
sudo flamegraph -F 99 -- target/release/vrp
