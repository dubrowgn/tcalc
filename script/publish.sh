#! /bin/bash

root_dir="$(realpath -L "$(dirname "$0")/..")"
cd "$root_dir" \
	|| exit

version="$(cat Cargo.toml | grep version | grep -Po '\d+\.\d+\.\d+')"
if [[ "$?" != "0" ]]; then
	echo "Failed to get package version"
	exit 1
fi

dist_dir="target/dist"
function pkg() {
	target="$1"
	ext=""

	if [[ "$target" =~ "linux" ]]; then
		os="lin"
	elif [[ "$target" =~ "windows" ]]; then
		os="win"
		ext=".exe"
	fi

	if [[ "$target" =~ "aarch64" ]]; then
		arch="arm64"
	elif [[ "$target" =~ "x86_64" ]]; then
		arch="x64"
	fi

	tar -C "target/$target/release/" -cO "tcalc$ext" \
		| zstd -T0 --ultra -22 \
		> "$dist_dir/tcalc-v$version-$os-$arch.tar.zst"
}

cargo test \
	&& ./script/build-all.sh \
	&& rm -rf "$dist_dir" \
	&& mkdir -p "$dist_dir" \
	&& pkg aarch64-unknown-linux-musl \
	&& pkg x86_64-pc-windows-gnu \
	&& pkg x86_64-unknown-linux-musl \
	&& cargo publish
