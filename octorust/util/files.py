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
from typing import List
from octorust.util.config import Config


def cleanup(cleanup_targets: List[str], config: Config):
    """
    Deletes all files or directories in a list, provided these actually exist
    :param cleanup_targets: The files and directories to delete
    :param config: The configuration used while compiling
    :return: None
    """
    if not config.keep:
        for cleanup_file in cleanup_targets:
            if os.path.isfile(cleanup_file):
                os.remove(cleanup_file)
        for cleanup_dir in cleanup_targets:
            if os.path.isdir(cleanup_dir):
                shutil.rmtree(cleanup_dir)
