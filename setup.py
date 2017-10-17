# imports
import os
from setuptools import setup, find_packages
from octorust.dependencies.gcc import download_sparc_elf_gcc
from octorust.dependencies.rust import copy_rust_libs
from octorust.dependencies.rust import compile_and_install_sparc_crates

# Create .octorust and irtss subdirectory
deps = os.path.join(os.path.expanduser("~"), ".octorust")
if not os.path.isdir(os.path.join(deps, "irtss")):
    os.makedirs(os.path.join(deps, "irtss"))

copy_rust_libs()
sparc_elf_gcc = download_sparc_elf_gcc()
compile_and_install_sparc_crates(sparc_elf_gcc)


setup(name="octorust",
      version="1.0.0",
      description="A OctoPOS rust compiler/build tool",
      author="Hermann Krumrey",
      author_email="hermann@krumreyh.com",
      packages=find_packages(),
      install_requires=[],
      test_suite='nose.collector',
      tests_require=['nose'],
      scripts=["bin/octorust"],
      zip_safe=False)
