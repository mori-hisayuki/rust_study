#!/bin/bash
set -eux

echo "Rust Update" >> ./post-start.log
rustup update >> ./post-start.log

echo "Rust Version" >> ./post-start.log
rustc --version >> ./post-start.log
