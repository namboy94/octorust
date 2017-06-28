"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys
from typing import List
from octorust.dependencies.general import dependency_check
from octorust.dependencies.gcc import determine_gcc, get_native_path


class Config(object):
    """
    A class that provides a collection of options needed in the compilation
    process.
    """

    def __init__(self, arch: str, variant: str, source: str, output: str,
                 mode: List[str]):
        """
        Creates a new Config object
        :param arch: The target architecture
        :param variant: The target variant
        :param source: The input source file
        :param output: The output file
        :param mode: The mode in which to run this program
        """
        self.arch = arch
        self.variant = variant
        self.source = source
        self.output = output
        self.mode = mode

        # Paths to dependencies locally installed by setup.py
        self.dependency_dir = os.path.join(
            os.path.expanduser("~"), ".octorust")
        self.octolib = os.path.join(self.dependency_dir, "octolib")
        self.libcore = os.path.join(self.octolib, "deps", "libcore")
        self.libc = os.path.join(self.octolib, "deps", "libc")

        # IRTSS paths
        self.irtss_release_path = os.path.join(
            self.dependency_dir, "irtss", self.arch, self.variant)
        self.irtss_include = os.path.join(self.irtss_release_path, "include")
        self.irtss_lib = os.path.join(self.irtss_release_path, "lib")
        self.irtss_sections_x = os.path.join(self.irtss_lib, "sections.x")

        # GCC and related paths
        self.gcc = determine_gcc(self.arch)  # Also checks the gcc dependency
        self.gcc_include = get_native_path(self.gcc, self.arch, "include")
        self.crti_o = get_native_path(self.gcc, self.arch, "crti.o")
        self.crtbegin_o = get_native_path(self.gcc, self.arch, "crtbegin.o")
        self.crtend_o = get_native_path(self.gcc, self.arch, "crtend.o")
        self.crtn_o = get_native_path(self.gcc, self.arch, "crtn.o")

        # Dependency check:
        depcheck = dependency_check([self.octolib, self.libcore, self.libc],
                                    self.irtss_release_path)

        if not depcheck[0] and self.mode[0].startswith("compile"):
            print(depcheck[1])
            sys.exit(1)
