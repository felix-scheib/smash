#!/bin/sh

if [ $# -ne 1 ]
then
  echo "USAGE: ./run_sender BINARY"
  exit
fi

qemu-system-x86_64 \
  -enable-kvm \
  -cpu host \
  -smp 1 -m 128M \
  -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
  -display none -serial stdio \
  -kernel hermit-loader-x86_64 \
  -initrd $1 \
  -netdev tap,id=net0,ifname=QemuTap0,script=no,downscript=no,vhost=on \
  -device virtio-net-pci,netdev=net0,disable-legacy=on,mac=52:54:00:21:34:56
