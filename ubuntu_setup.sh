#!/bin/bash
# Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>
#
# This file is part of octorust.
#
# octorust is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# octorust is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with octorust.  If not, see <http://www.gnu.org/licenses/>.

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
