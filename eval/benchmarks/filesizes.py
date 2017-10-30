#!/usr/bin/env python3

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