# imports
import os
from setuptools import setup, find_packages
from octorust.dependencies.gcc import download_sparc_elf_gcc

download_sparc_elf_gcc()

# Create irtss subdirectory
deps = os.path.join(os.path.expanduser("~"), ".octorust")
if not os.path.isdir(os.path.join(deps, "irtss")):
    os.makedirs(os.path.join(deps, "irtss"))

setup(name="octorust",
      version="0.0.1",
      description="A OctoPOS rust compiler/build tool",
      author="Hermann Krumrey",
      author_email="hermann@krumreyh.com",
      packages=find_packages(),
      install_requires=[],
      test_suite='nose.collector',
      tests_require=['nose'],
      scripts=["bin/octorust"],
      zip_safe=False)
