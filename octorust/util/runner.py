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
