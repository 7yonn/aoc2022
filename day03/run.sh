#!/bin/sh

set -xe

rustc -o part2 part2.rs
time ./part2
