"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
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
