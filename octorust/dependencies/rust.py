import os
import toml
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


def compile_and_install_crates(sparc_gcc_path: str):
    """
    Compiles the core and libc crates and installs them in the local rustup
    toolchain installation directory for the SPARC LEON architecture
    :param sparc_gcc_path: The path to the sparc-elf-gcc executable
    :return: None
    """
    os.chdir("octolib/deps/depcompile")
    generate_leon_specification(sparc_gcc_path)

    for target in [
        "leon",
        "i686-unknown-linux-gnu",
        "x86_64-unknown-linux-gnu"
    ]:

        # Ugly hack to keep liballoc and liballoc_system out for leon
        if target == "leon":
            shutil.copyfile("Cargo.toml", "Cargo.toml.backup")
            with open("Cargo.toml", 'r') as f:
                cargo_toml = toml.loads(f.read())
            cargo_toml["dependencies"].pop("alloc")
            cargo_toml["dependencies"].pop("alloc_system")
            with open("Cargo.toml", 'w') as f:
                f.write(toml.dumps(cargo_toml))

        Popen(["cargo", "rustc", "--target", target, "--release"]).wait()

        if os.path.isfile("Cargo.toml.backup"):
            if os.path.isfile("Cargo.toml"):
                os.remove("Cargo.toml")
            os.rename("Cargo.toml.backup", "Cargo.toml")

        rustup_output = check_output(["rustup", "show"]).decode("utf-8")
        rustup_toolchain = rustup_output.rsplit("-----", 1)[1].strip()
        rustup_toolchain = rustup_toolchain.split("\n")[0].split(
            " (default)")[0]
        rustup_install_path = os.path.join(
            os.path.expanduser("~"),
            ".rustup",
            "toolchains",
            rustup_toolchain,
            "lib/rustlib",
            target,
            "lib"
        )

        if not os.path.isdir(rustup_install_path):
            os.makedirs(rustup_install_path)

        for existing_dep in os.listdir(rustup_install_path):

            for new_dep in ["libcore",
                            "liblibc",
                            "liballoc",
                            "liballoc_system"]:
                if new_dep in existing_dep:
                    to_delete = os.path.join(rustup_install_path, existing_dep)
                    if os.path.isfile(to_delete):
                        os.remove(to_delete)

        for dep in os.listdir("target/" + target + "/release/deps"):
            if "debcompile" in dep:
                continue
            dep_path = os.path.join("target/" + target + "/release/deps", dep)
            dest_path = os.path.join(rustup_install_path, dep)
            os.rename(dep_path, dest_path)

    os.remove("leon.json")
    os.chdir("../../..")
