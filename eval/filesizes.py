#!/usr/bin/env python3
"""
Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>

This file is part of octorust.

octorust is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

octorust is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with octorust.  If not, see <http://www.gnu.org/licenses/>.
"""

import os


commands = {
	"Rust": "octorust -i 2017-06-07 startup/startup -o out",
	"Rust-opt": "octorust -i 2017-06-07 startup/startup -o out --release",
	"C": "octorust -i 2017-06-07 startup/startup.c -o out",
	"C-opt": "octorust -i 2017-06-07 startup/startup.c -o out --release",
	"X10": "x10firm startup/Startup.x10 -mtarget=i686-invasic-irtss -o out",
	"X10-opt": "x10firm startup/Startup.x10 -mtarget=i686-invasic-irtss -o out -O3"
}

output = ""

for language, command in commands.items():
	print(language)
	if os.path.isfile("out"):
		os.remove("out")
	os.system(command)
	output += language + ": " + str(os.path.getsize("out")) + "\n"

if os.path.isfile("out"):
		os.remove("out")

print(output)
with open("temci_output/filesizes", "w") as f:
	f.write(output)