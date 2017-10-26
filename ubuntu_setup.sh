#!/bin/bash

echo "Installing python 3 and setuptools"
sudo apt install python3 python3-setuptools python3-pip

echo "Installing gcc-multilib"
sudo apt install gcc-multilib

echo "Installing curl"
sudo apt install curl

echo "Installing rustup"
curl https://sh.rustup.rs -sSf | sh
export PATH=$PATH:~/.cargo/bin
rustup install nightly-2017-06-01
rustup default nightly-2017-06-01
rustup target install i686-unknown-linux-gnu

echo "Installing octorust"
pip3 install toml --user
python3 setup.py install --user
echo 'PATH=$PATH:~/.local/bin' >> ~/.bashrc
export PATH=$PATH:~/.local/bin

echo "If no .netrc is set up, fetching IRTSS builds will fail"
