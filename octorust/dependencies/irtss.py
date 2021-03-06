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

import os
import shutil
from subprocess import check_output, Popen


def get_irtss_release(
        release_path: str, build_version: str, arch: str, variant: str):
    """
    Downloads an IRTSS release and stores it in the local .octorust
    directory.
    :param release_path: The path to the locally stored irtss release directory
    :param build_version: The version to fetch
    :param arch: The architecture for which to fetch a release
    :param variant: The variant for which to fetch a release
    :return: None
    """

    # Only fetch release if it doesn't exist yet
    if not os.path.isdir(release_path):

        release = download_irtss_release(build_version, arch, variant)

        parent_dir = os.path.dirname(release_path)
        if not os.path.isdir(parent_dir):
            os.makedirs(parent_dir)

        os.rename(os.path.join(release, arch, variant), release_path)
        shutil.rmtree(release)
        print("IRTSS release successfully downloaded.")

    else:
        print("IRTSS release already exists.")


def download_irtss_release(build_version: str, arch: str, variant: str) -> str:
    """
    Retrieves an irtss release from the invasic release site
    This requires a .netrc file in the user's home directory with the
    format:

    machine www4.cs.fau.de
    login <username>
    password <password>

    Of course, valid login credentials are required
    :param build_version: The version of IRTSS to fetch
    :param arch: The architecture for which to download the release
    :param variant: The variant for which to download the release
    :return: The release directory path
    """
    snap = os.listdir(".")

    base_url = "https://www4.cs.fau.de/invasic/octopos/"

    if build_version == "current":

        filename = "release." + arch + "." + variant + ".current.txt"
        url = base_url + filename

        filename = str(check_output(
            ["wget", "-nv", "--no-check-certificate", "-O", "-", url]
        )).split("b'", 1)[1].rsplit("\\n'", 1)[0]

    else:
        filename = "release." + arch + "." + variant + \
                   "." + build_version + ".tar.bz2"

    url = base_url + filename

    Popen(["wget", "-nv", "--no-check-certificate", "-O", filename, url])\
        .wait()
    Popen(["tar", "xjf", filename]).wait()
    os.remove(filename)
    os.remove("current")

    release = list(filter(lambda x: x not in snap, os.listdir(".")))[0]
    return release
