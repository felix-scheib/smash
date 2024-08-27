#!/bin/sh

GUESTFS=guestfs

mkdir -p /tmp/$GUESTFS
cp ./config.yml /tmp/$GUESTFS

sudo whoami
sudo ./virtiofsd --socket-path=/tmp/vhostqemu --shared-dir=/tmp/$GUESTFS &

sleep 1

sudo chmod 777 /tmp/vhostqemu
