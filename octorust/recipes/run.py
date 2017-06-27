"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

from subprocess import Popen
from octorust.config import Config


def run_executable(config: Config):
    """
    Executes the compiled application
    :param config: The configuration used when compiling
    :return: None
    """

    arch = config.arch
    app = config.out

    if arch == "x86guest":
        Popen(["./" + app]).wait()
    elif arch == "x64native":
        Popen(["qemu-system-x86_64",
               "-serial", "stdio", "-smp", "8",
               "-numa", "node", "-numa", "node", "-m", "1G",
               "-no-reboot", "-display", "none", "-kernel", app]).wait()
    elif arch == "leon":
        Popen(["qemu-sparc", app]).wait()
