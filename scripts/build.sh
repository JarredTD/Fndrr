#!/bin/bash

rm -rf target/

targets=(
    "aarch64-apple-darwin"   # Mac M1 (ARM64)
    "x86_64-apple-darwin"    # Mac Intel (x86_64)
    "x86_64-pc-windows-gnu"  # Windows (GNU)
    "x86_64-unknown-linux-gnu-local" # Linux (x86_64)
)

for target in "${targets[@]}"
do
    echo "Building for $target..."
    cargo build --target $target --release
    if [ $? -eq 0 ]; then
        echo "Build for $target completed successfully."
    else
        echo "Build for $target failed."
        continue
    fi
done
