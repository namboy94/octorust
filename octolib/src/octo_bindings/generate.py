#!/usr/bin/env python

import os
import sys
from subprocess import Popen

try:
    dir = sys.argv[1]
except IndexError:
    dir = "../../irtss/2017-06-13/x86guest/generic"

api_dir = os.path.join(dir, "include")

for header in os.listdir(api_dir):

    header_file = os.path.join(api_dir, header)
    header_rs_file = header.rsplit(".", 1)[0] + ".rs"

    if header.endswith(".h"):
        Popen(["bindgen", header_file, "-o", header_rs_file])
