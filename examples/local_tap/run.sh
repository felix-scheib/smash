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
  -chardev socket,id=char0,path=/tmp/vhostqemu \
  -device vhost-user-fs-pci,queue-size=1024,chardev=char0,tag=root \
  -object memory-backend-file,id=mem,size=1G,mem-path=/dev/shm,share=on \
  -numa node,memdev=mem \
  -netdev tap,id=net0,ifname=QemuTap0,script=no,downscript=no,vhost=on \
  -device virtio-net-pci,netdev=net0,disable-legacy=on,mac=52:54:00:21:34:56 \
  -append "-- LOG=trace CONFIG=/root/config.yml"
