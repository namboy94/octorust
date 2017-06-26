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

        # Dependencies
        self.dependency_dir = os.path.join(os.path.expanduser("~"), ".octorust")

        self.octolib = os.path.join(self.dependency_dir, "octolib")
        self.libcore = os.path.join(self.octolib, "deps", "libcore")
        self.libc = os.path.join(self.octolib, "deps", "libc")

        # IRTSS
        target = self.arch + "/" + self.variant
        irtss_base_path = os.path.join(self.dependency_dir, "releases/current/")
        self.irtss_release_path = os.path.join(irtss_base_path, target)

        self.irtss_include = os.path.join(self.irtss_release_path, "include")
        self.irtss_lib = os.path.join(self.irtss_release_path, "lib")
        self.irtss_sections_x = os.path.join(self.irtss_lib, "sections.x")

        self.check_dependencies()

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

    def check_dependencies(self):
        """
        Checks if all dependencies are present. If not, the program exits.
        :return: None
        """
        for dependency in [self.octolib,
                           self.libcore,
                           self.libc,
                           self.irtss_release_path]:
            if not os.path.isdir(dependency):
                print("Dependency " + dependency + "' not sattisfied.")
                print("Please install this dependency or run\n"
                      "'./compile.py depgen'\n"
                      "from the octorust source directory")
                sys.exit(1)