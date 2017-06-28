"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

from subprocess import Popen


def run_executable(executable: str, arch: str):
    """
    Executes the compiled application
    :param executable: The executable to execute
    :param arch: The architecture on which the program will be executed
    :return: None
    """

    if arch == "x86guest":
        Popen(["./" + executable]).wait()
    elif arch == "x64native":
        Popen(["qemu-system-x86_64",
               "-serial", "stdio", "-smp", "8",
               "-numa", "node", "-numa", "node", "-m", "1G",
               "-no-reboot", "-display", "none", "-kernel", executable]).wait()
    elif arch == "leon":
        Popen(["qemu-sparc", executable]).wait()
    else:
        print("Unsupported Architecture. Can't run application")
