# AArch64_rust_os
# a simple rust os for aarch64
# Github://github.com/0x4248/AArch64_rust_os
# By: 0x4248

all: build run

build:
	cargo xbuild --target aarch64-unknown-none.json

run:
	qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none/debug/aarch64-bare-metal