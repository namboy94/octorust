#!/bin/bash

echo "Installing python 3 and setuptools"
sudo apt install python3 python3-setuptools

echo "Installing gcc-multilib"
sudo apt install gcc-multilib

echo "Installing rustup"
curl https://sh.rustup.rs -sSf | sh
export PATH=$PATH:~/.cargo/bin
rustup install nightly-2017-06-01
rustup default nightly-2017-06-01

echo "Installing octorust"
python3 setup.py install --user
echo 'PATH=$PATH:~/.local/bin' >> ~/.bashrc
export PATH=$PATH:~/.local/bin

echo "If no .netrc is set up, fetching IRTSS builds will fail"
