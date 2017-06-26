import os
from subprocess import check_output, Popen


def get_irtss_release(arch: str, variant: str) -> str:
    """
    Retrieves an irtss release from the invasic release site
    
    This requires a .netrc file in the user's home directory with the
    format:
    
    machine www4.cs.fau.de
    login <username>
    password <password>
    
    Of course, valid login credentials are required
    :param arch: The architecture for which to download the release
    :param variant: The variant for which to download the release
    :return: The release directory path
    """
    snap = os.listdir(".")

    base_url = "https://www4.cs.fau.de/invasic/octopos/"
    filename = "release." + arch + "." + variant + ".current.txt"
    url = base_url + filename

    filename = str(check_output(
        ["wget", "-nv", "--no-check-certificate", "-O", "-", url]
    )).split("b'", 1)[1].rsplit("\\n'", 1)[0]

    url = base_url + filename

    Popen(["wget", "-nv", "--no-check-certificate", "-O", filename, url]).wait()
    Popen(["tar", "xjf", filename]).wait()
    os.remove(filename)
    os.remove("current")

    release = list(filter(lambda x: x not in snap, os.listdir(".")))[0]
    return release
