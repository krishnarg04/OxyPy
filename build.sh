#!/usr/bin/env bash
set -e
binary='OxyPy'
# Linux (static)
cargo build --release --target x86_64-unknown-linux-musl
mkdir -p dist/linux
cp target/x86_64-unknown-linux-musl/release/$binary dist/linux/
tar -czvf $binary-linux-x86_64.tar.gz -C dist/linux . 

# Windows
cargo build --release --target x86_64-pc-windows-gnu
mkdir -p dist/windows
cp target/x86_64-pc-windows-gnu/release/$binary.exe dist/windows/ 
zip -r $binary-windows-x86_64.zip dist/windows 

