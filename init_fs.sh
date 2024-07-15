#!/bin/sh

#if [ "$EUID" -ne 0 ]
#  then echo "Please run as root"
#  exit
#fi

mkdir -p /tmp/guestfs
cp ./config.yml /tmp/guestfs

sudo whoami
sudo ./virtiofsd --socket-path=/tmp/vhostqemu --shared-dir=/tmp/guestfs &

sleep 1

sudo chmod 777 /tmp/vhostqemu
