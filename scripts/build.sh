#!/bin/bash

clear
rm -rf target/

base_name="fndrr"

declare -a targets=("x86_64-pc-windows-gnu" "x86_64-apple-darwin" "aarch64-apple-darwin" "x86_64-unknown-linux-gnu")
declare -a platforms=("windows" "macos-intel" "macos-arm" "linux")

length=${#targets[@]}
for ((i = 1; i <= ${length}; i++)); do
    target=${targets[$i]}
    platform=${platforms[$i]}
    echo "Building for $target ($platform)..."
    cargo build --target "$target" --release

    file_ext=""
    if [[ "$target" == *"windows-gnu"* ]]; then
        file_ext=".exe"
    fi

    original_binary_name="${base_name}${file_ext}"
    new_binary_name="${base_name}-${platform}${file_ext}"

    mv "target/${target}/release/${original_binary_name}" "target/${target}/release/${new_binary_name}"
done
