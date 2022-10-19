#! /bin/bash

targets=(
	x86_64-unknown-linux-musl
	aarch64-unknown-linux-musl
	x86_64-pc-windows-gnu
)

for target in "${targets[@]}"; do
	echo "Building '$target'..."
	cargo build --release --target "$target" \
		|| exit
done
