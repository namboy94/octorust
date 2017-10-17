import os
import shutil
from subprocess import Popen, check_output
from octorust.recipes.rust import generate_leon_specification


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


def compile_and_install_sparc_crates(sparc_gcc_path: str):
    """
    Compiles the core and libc crates and installs them in the local rustup
    toolchain installation directory for the SPARC LEON architecture
    :param sparc_gcc_path: The path to the sparc-elf-gcc executable
    :return: None
    """
    os.chdir("octolib/deps/depcompile")
    generate_leon_specification(sparc_gcc_path)
    Popen(["cargo", "rustc", "--target", "leon", "--release"]).wait()
    os.remove("leon.json")

    rustup_output = check_output(["rustup", "show"]).decode("utf-8")
    rustup_toolchain = rustup_output.rsplit("-----", 1)[1].strip()
    rustup_toolchain = rustup_toolchain.split("\n")[0].split(" (default)")[0]
    rustup_install_path = os.path.join(
        os.path.expanduser("~"),
        ".rustup",
        "toolchains",
        rustup_toolchain,
        "lib/rustlib/leon/lib"
    )

    if not os.path.isdir(rustup_install_path):
        os.makedirs(rustup_install_path)

    for existing_dep in os.listdir(rustup_install_path):
        if "libcore" in existing_dep or "liblibc" in existing_dep:
            os.remove(os.path.join(rustup_install_path, existing_dep))

    for dep in os.listdir("target/leon/release/deps"):
        if "debcompile" in dep:
            continue
        dep_path = os.path.join("target/leon/release/deps", dep)
        dest_path = os.path.join(rustup_install_path, dep)
        os.rename(dep_path, dest_path)

    os.chdir("../../..")
