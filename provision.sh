#!/bin/bash

# This is the provisioning script which is executed when the virtual machine is first created.
# Here is where we install all of the dependencies for our project.

apt-get update

# Developer tools.
apt-get -y install git
apt-get -y install curl
apt-get -y install ruby

# Install the pre-built GNU ARM tools (including the compiler).
apt-get -y install gdb-arm-none-eabi
apt-get -y install openocd
apt-get -y install qemu-system-arm

# Install nightly rust. Do this as the vagrant user so it installs in the right place. 
sudo -u vagrant HOME=/home/vagrant bash -c "curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=nightly"

# Need these to install xargo.
apt-get -y install gcc
apt-get -y install cmake
apt-get -y install libssl-dev

# Need to log out/in first for cargo to be on path.
# Note this doesn't work:  error: no default toolchain configured

/home/vagrant/.cargo/bin/cargo install xargo

# Create a configuration file in /etc/udev/rules.d to allow access to the USB device needed for programming.
PROG_CONF_FILE=/etc/udev/rules.d/tiva-c-launchpad.rules
echo 'SUBSYSTEM=="usb", ATTR{idVendor}=="1cbe", ATTR{idProduct}=="00fd", OWNER="vagrant", MODE="0666"' > $PROG_CONF_FILE

# Create a script in /etc/profile.d which will automatically be run upon login. We'll set our path
# and any other environment variables here.
SETUP_SCRIPT=/etc/profile.d/setup-environment.sh
#echo "export PATH=$PATH:/usr/local/lm4tools/lm4flash" >  $SETUP_SCRIPT

# Have the script start us in the host-shared folder.
echo "cd /vagrant" > $SETUP_SCRIPT
