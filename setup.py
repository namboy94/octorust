# imports
import os
import shutil
from setuptools import setup, find_packages
from octorust.config import Config
from subprocess import Popen


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

#  Custom dependency installation
deps = os.path.join(os.path.expanduser("~"), ".octorust")
octolib_dep = os.path.join(deps, "octolib")

if os.path.isdir(octolib_dep):
    shutil.rmtree(octolib_dep)
shutil.copytree("octolib", octolib_dep)

if not os.path.isdir(os.path.join(deps, "irtss-current")):
    os.makedirs(os.path.join(deps, "irtss-current"))

toolchain_dir = os.path.join(os.path.expanduser("~"),
                             ".octorust", "toolchains")
if not Config.check_if_in_path("sparc-elf-gcc") \
        and not os.path.isdir(os.path.join(toolchain_dir, "sparc-elf")):

    if not os.path.isdir(toolchain_dir):
        os.makedirs(toolchain_dir)

    gcc_url = "https://www4.cs.fau.de/invasic/tools/" \
              "sparc-elf-7.1.0-x86_64.tar.bz2"
    Popen(["wget", gcc_url]).wait()
    Popen(["tar", "xjfv", "sparc-elf-7.1.0-x86_64.tar.bz2"]).wait()
    os.remove("sparc-elf-7.1.0-x86_64.tar.bz2")
    os.rename("sparc-elf-7.1.0", os.path.join(toolchain_dir, "sparc-elf"))

    with open(os.path.join(os.path.expanduser("~"), ".bashrc"), 'a') as f:
        f.write("PATH=$PATH:$HOME/.octorust/toolchains/sparc-elf/bin/")
