"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
from typing import List


def cleanup(cleanup_files: List[str]):
    """
    Deletes all files in a list, provided these files actually exist
    :param cleanup_files: The files to delete
    :return: None
    """
    for cleanup_file in cleanup_files:
        if os.path.isfile(cleanup_file):
            os.remove(cleanup_file)
