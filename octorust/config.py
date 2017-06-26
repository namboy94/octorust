"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys


class Config(object):
    """
    A class that provides a collection of options needed in the compilation
    process.
    """

    def __init__(self, arch: str, variant: str, source: str, out: str):
        """
        Creates a new Config object
        :param arch: The target architecture
        :param variant: The target variant
        :param source: The input source file
        :param out: The output file
        """
        self.arch = arch
        self.variant = variant
        self.source = source
        self.out = out

        # Generate names
        self.rust_static_lib = "lib" + self.source.rsplit(".rs", 1)[0] + ".a"
        self.c_file = "dummy.c"
        self.c_object = "dummy.o"

        # GCC
        self.gcc = {
            "x86guest": "gcc",
            "x64native": "gcc",
            "leon": "sparc-elf-gcc"}[self.arch]

        if not self.check_if_in_path(self.gcc):
            print("gcc '" + self.gcc + "' not in path. Can not continue.")
            sys.exit(1)

        # TODO Find dynamically
        self.c_include = "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/include"

        # IRTSS  TODO Make this more configurable
        target = self.arch + "/" + self.variant
        self.irtss_release_path = "../../releases/current/" + target
        self.irtss_include = os.path.join(self.irtss_release_path, "include")
        self.irtss_lib = os.path.join(self.irtss_release_path, "lib")
        self.irtss_sections_x = os.path.join(self.irtss_lib, "sections.x")

    @staticmethod
    def check_if_in_path(command: str) -> bool:
        """
        Checks if a given command is currently found in the system's path.
        :param command: The command to check
        :return: True if the command is in the path, False otherwise
        """

        for path in os.environ["PATH"].split(":"):
            if os.path.isfile(os.path.join(path, command)):
                return True
        return False
