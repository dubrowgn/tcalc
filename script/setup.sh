#! /bin/bash

sudo apt-get update \
	&& sudo apt-get install \
		gcc-aarch64-linux-gnu \
		gcc-mingw-w64-x86-64 \
		zstd \
	|| exit

rustup target add \
	x86_64-pc-windows-gnu \
	aarch64-unknown-linux-musl \
	x86_64-unknown-linux-musl
