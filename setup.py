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

# imports
import os
import sys
from setuptools import setup, find_packages

if sys.version_info.major < 3:
    print("Only python 3+ is supported")
    sys.exit(1)

# Only import python3 modules after checking that python 3 is used
from octorust.dependencies.gcc import download_sparc_elf_gcc
from octorust.dependencies.rust import copy_rust_libs
from octorust.dependencies.rust import compile_and_install_crates

# Create .octorust and irtss subdirectory
deps = os.path.join(os.path.expanduser("~"), ".octorust")
if not os.path.isdir(os.path.join(deps, "irtss")):
    os.makedirs(os.path.join(deps, "irtss"))

copy_rust_libs()
sparc_elf_gcc = download_sparc_elf_gcc()
compile_and_install_crates(sparc_elf_gcc)


setup(name="octorust",
      version="1.0.0",
      description="A OctoPOS rust compiler/build tool",
      author="Hermann Krumrey",
      author_email="hermann@krumreyh.com",
      packages=find_packages(),
      install_requires=["toml"],
      test_suite='nose.collector',
      tests_require=['nose'],
      scripts=["bin/octorust"],
      zip_safe=False)
