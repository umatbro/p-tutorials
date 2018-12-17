#!/bin/sh
RELEASE=3.7.1
 
# install dependencies
sudo apt-get update && apt-get upgrade
sudo apt-get install libbz2-dev liblzma-dev libsqlite3-dev libncurses5-dev libgdbm-dev zlib1g-dev libreadline-dev libssl-dev tk-dev libffi-dev
 
# download and build Python
mkdir ~/python3
cd ~/python3
wget https://www.python.org/ftp/python/$RELEASE/Python-$RELEASE.tar.xz
tar xvf Python-$RELEASE.tar.xz
cd Python-$RELEASE
./configure
make
sudo make install
sudo rm -rf ~/python3
cd ~
