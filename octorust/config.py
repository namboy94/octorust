"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys
import shutil
from subprocess import check_output
from octorust.irtss import get_irtss_release


class Config(object):
    """
    A class that provides a collection of options needed in the compilation
    process.
    """

    def __init__(self, arch: str, variant: str, source: str, out: str,
                 run: bool):
        """
        Creates a new Config object
        :param arch: The target architecture
        :param variant: The target variant
        :param source: The input source file
        :param out: The output file
        :param run: Flag that executes the file upon compilation
        """
        self.arch = arch
        self.variant = variant
        self.source = source
        self.out = out
        self.run = run

        # Generate names
        base_name = str(os.path.basename(self.source))
        self.rust_static_lib = "lib" + base_name.rsplit(".rs", 1)[0] + ".a"
        self.c_file = "dummy.c"
        self.c_object = "dummy.o"

        # GCC
        self.gcc = {
            "x86guest": "gcc",
            "x64native": "gcc",
            "leon": "sparc-elf-gcc"}[self.arch]

        if not self.check_if_in_path(self.gcc):

            sparc_elf = os.path.join(os.path.expanduser("~"),
                                     ".octorust", "toolchains", "sparc-elf")
            if self.arch == "leon" and os.path.isdir(sparc_elf):
                self.gcc = os.path.join(sparc_elf, "bin", "sparc-elf-gcc")
            else:
                print("gcc '" + self.gcc + "' not in path. Can not continue.")
                sys.exit(1)

        # Dependencies
        self.dependency_dir = os.path.join(os.path.expanduser("~"),
                                           ".octorust")

        self.octolib = os.path.join(self.dependency_dir, "octolib")
        self.libcore = os.path.join(self.octolib, "deps", "libcore")
        self.libc = os.path.join(self.octolib, "deps", "libc")

        # IRTSS
        target = self.arch + "/" + self.variant
        irtss_base_path = os.path.join(self.dependency_dir, "irtss-current")
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
        IRTSS builds are however fetched automatically
        :return: None
        """
        for dependency in [self.octolib,
                           self.libcore,
                           self.libc]:
            if not os.path.isdir(dependency):
                print("Dependency " + dependency + "' not satisfied.")
                print("Please reinstall octorust.")
                sys.exit(1)

        if not os.path.isdir(self.irtss_release_path):
            self.download_irtss()

    def download_irtss(self):
        """
        Downloads the latest IRTSS release for the target architecture and
        variant.
        :return: None
        """

        release = get_irtss_release(self.arch, self.variant)

        if not os.path.isdir(os.path.dirname(self.irtss_release_path)):
            os.makedirs(os.path.dirname(self.irtss_release_path))

        os.rename(os.path.join(release, self.arch, self.variant),
                  self.irtss_release_path)

        shutil.rmtree(release)

    def get_native_path(self, target: str) -> str:
        """
        Queries gcc for a native object or include path
        :param target: The target whose path should be found
        :return: The path of the target
        """

        command = [self.gcc, "--print-file-name", target]
        if self.arch == "x86guest":
            command.append("-m32")

        return check_output(command).decode().strip()
