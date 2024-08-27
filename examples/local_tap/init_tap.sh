#!/bin/sh

# add a net bridge named Gefyra0
sudo ip link add Bridge0 type bridge
sudo ip link set Bridge0 up

# add tun/tap interface for VM
sudo ip tuntap add QemuTap0 mode tap
sudo ip addr add 10.0.5.1/24 broadcast 10.0.5.255 dev QemuTap0

sudo ip link set QemuTap0 up

# connect tun/tap interface with brigde
sudo ip link set QemuTap0 master Bridge0
