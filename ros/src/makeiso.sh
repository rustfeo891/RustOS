#!/bin/bash

set -e
#i686-elf-as boot.s -o boot.o
#cp boot.o ..
cargo build --release

KERNEL=/home/fe2o/mis/os/RustOS/ros/target/target/release/ros 

echo "---------debug-------"
echo "where: $KERNEL"
readelf -S "$KERNEL" | grep -A1 multiboot || true
hexdump -C "$KERNEL" | grep -m1 "b0 02 ad 1b" || true
echo "---------------------"

rm -rf isodir
mkdir -p isodir/boot/grub
cp "$KERNEL"  .
cp "$KERNEL" isodir/boot/myos
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue -o ros.iso isodir
