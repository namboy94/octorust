import os
import sys
import shutil
from subprocess import Popen
from octorust.util.files import cleanup
from octorust.util.config import Config
from octorust.linking.octopos_link import link_app
from octorust.recipes.rust import get_rust_target_triple,\
    generate_leon_specification


def compile_using_cargo(config: Config):

    static_lib = compile_static_library(config)
    c_object = compile_c_object(config)
    link_app(config, [c_object, static_lib])
    cleanup([c_object, static_lib])


def compile_static_library(config: Config) -> str:
    """
    Compiles a rust crate project using cargo and automatically
    includes octolib. The project is compiled as a static library
    for easy integration with octopos
    :param config: The compilation configuration
    :return: The path to the generated library
    """

    current = os.getcwd()
    os.chdir(config.source)

    generate_cargo_toml(config)
    crate_name = read_crate_name_from_cargo_toml("Cargo.toml")

    target_triple = get_rust_target_triple(config.arch)
    command = ["cargo", "rustc", "--", "--target", target_triple]

    if config.arch == "leon":
        generate_leon_specification(config)
        command += ["-C", "link-dead-code"]

    libname = "lib" + crate_name + ".a"
    # For x64, cargo doesn't create a separate build directory
    if target_triple == "x86_64-unknown-linux-gnu":
        output = os.path.join("target", "debug", libname)
    else:
        output = os.path.join("target", target_triple, "debug", libname)

    print(command)
    Popen(command).wait()

    if not os.path.isfile(output):
        print("Failed to compile the static rust library")
        sys.exit(1)

    os.rename(output, os.path.join(current, libname))
    cleanup(["leon.json", "Cargo.toml"])

    os.chdir(current)
    return libname


def generate_cargo_toml(config: Config):
    """
    Generates a Cargo.toml file from the crate's OctoCargo.toml file
    :param config: The config to use while compiling
    :return: None
    """

    shutil.copyfile("OctoCargo.toml", "Cargo.toml")

    with open("Cargo.toml", 'a') as octocargo:
        octocargo.write(
            "core = { path = \"" + config.libcore + "\" }\n" +
            "octolib = { path = \"" + config.octolib + "\" }"
        )


def read_crate_name_from_cargo_toml(cargo_toml: str) -> str:
    """
    Reads the crate name of a rust project from the associated
    Cargo.toml file
    :param cargo_toml: The Cargo.toml file
    :return: The crate name
    """

    with open(cargo_toml, 'r') as cargo:
        lines = cargo.read().split("\n")

    for line in lines:
        if "name=" in line.replace(" ", ""):
            return line.split("=", 1)[1].strip()

    print("Cargo.toml does not contain a project name")
    sys.exit(1)


def compile_c_object(config: Config) -> str:
    """
    Compiles A C file that acts as a compatibility layer to OctoPOS
    :param config: The config that defines the compiler options
    :return: The path to the generated object file
    """

    generate_c_dummy("dummy.c")

    command = [config.gcc]

    if config.arch.startswith("x"):

        if config.arch == "x86guest":

            command += [
                "-mfpmath=sse",
                "-msse2",
                "-m32",
                "-nostdinc",
                "-fno-asynchronous-unwind-tables",
                "-fno-stack-protector",
            ]

        elif config.arch == "x64native":

            command += [
                "-m64",
                "-fno-stack-protector",
                "-mno-red-zone",
                "-nodefaultlibs",
                "-nostdlib",
                "-mcx16",
                "-D__STDC_LIMIT_MACROS",
                "-O3",
                "-nostdinc",
                "-fno-asynchronous-unwind-tables",
                "-fno-stack-protector",
            ]

        command += [  # Same for x86guest and x64native
            "-I" + config.irtss_include,
            "-isystem",
            config.gcc_include,
            "-D__OCTOPOS__",
            "-std=gnu11"
        ]

    elif config.arch == "leon":

        command += [
            "-mcpu=v8",
            "-O3",
            "-nostdinc",
            "-fno-asynchronous-unwind-tables",
            "-fno-stack-protector",
            "-I" + config.irtss_include,
            "-isystem",
            config.gcc_include,
            "-D__OCTOPOS__",
            "-std=gnu99"

        ]

    command += ["-c", "dummy.c", "-o", "dummy.o"]
    print(command)
    Popen(command).wait()

    cleanup(["dummy.c"])

    if not os.path.isfile("dummy.o"):
        print("C Object did not compile. Can not continue.")
        sys.exit(1)

    return "dummy.o"


def generate_c_dummy(destination: str):
    """
    Writes a dummy C file that acts as 'glue' to get rust working with
    OctoPOS
    :param destination: The dummy file location
    :return: None
    """

    data = "#include <stdint.h>\n" \
           "void main_rust_ilet(uint8_t claim_t);"

    with open(destination, 'w') as dummy:
        dummy.write(data)
