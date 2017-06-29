import os
import shutil


def copy_rust_libs():
    """
    Copies the octolib directory into the .octorust directory
    :return: None
    """
    deps = os.path.join(os.path.expanduser("~"), ".octorust")

    octolib_dep = os.path.join(deps, "octolib")

    if os.path.isdir(octolib_dep):
        shutil.rmtree(octolib_dep)
    shutil.copytree("octolib", octolib_dep)
