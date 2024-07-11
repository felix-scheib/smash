#!/bin/sh

if [ $# -ne 1 ]
then
  echo "Usage: ./run_user.sh BINARY"
  exit
fi

qemu-system-x86_64 \
  -enable-kvm \
  -cpu host \
  -smp 1 -m 1G \
  -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
  -display none -serial stdio \
  -kernel hermit-loader-x86_64 \
  -initrd $1 \
  -append "-- LOG_LEVEL=trace CONFIG_FILE=/root/config.yml"
