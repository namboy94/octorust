import os
from typing import List, Tuple


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


def dependency_check(directories: List[str], irtss_release_path: str = "") \
        -> Tuple[bool, str, str]:
    """
    Checks if various dependencies are satisfied
    :param directories: Directories to check
    :param irtss_release_path: The IRTSS release path. Can be left blank,
                               in which case it will simply be ignored
    :return: A tuple of a boolean and two strings, with the boolean indicating
             if all dependencies are accounted for, and the strings consisting
             of a the name of the first found missing dependency and the second
             string containing a potential error message in case a
             dependency is missing
    """

    for dependency in directories:
        if not os.path.isdir(dependency):

            message = "Dependency " + dependency + "' not satisfied.\n"
            message += "Please reinstall octorust."
            return False, dependency, message

    # IRTSS release check must be last
    if not os.path.isdir(irtss_release_path) and irtss_release_path != "":
        return False, irtss_release_path, \
               "No local IRTSS release found. Use --fetch-irtss to download " \
               "the corresponding IRTSS release for " \
               "your architecture and variant."

    return True, "", ""
