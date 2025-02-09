#!/bin/bash

KERNEL=target/riscv64gc-unknown-none-elf/release/tcore-kernel
IMG=fat32.img
cargo xtask make --oscomp --pack-target test/oscomp/judge/  --pack-image fat32.img --dump  --build-test --log trace
tmux new-session -d "qemu-system-riscv64 -machine virt -m 2G -nographic -bios default -kernel $KERNEL -drive file=$IMG,if=none,format=raw,id=x0 -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -s -S" && \
tmux split-window -h "riscv64-unknown-elf-gdb -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
tmux -2 attach-session -d
