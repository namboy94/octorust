import os
import sys
import shutil
from subprocess import Popen
from octorust.util.files import cleanup
from octorust.util.config import Config
from octorust.linking.octopos_link import link_app
from octorust.recipes.c import compile_c_object
from octorust.recipes.rust import get_rust_target_triple,\
    generate_leon_specification


def compile_using_cargo(config: Config):
    """
    Compiles a rust crate using cargo. This crate needs to use an
    'OctoCargo.toml' file instead of the normal 'Cargo.toml' file
    :param config: The configuration to use while compiling
    :return: None
    """

    static_lib = compile_static_library(config)
    c_file = generate_c_dummy()
    c_object = compile_c_object(config, c_file)
    link_app(config, [c_object, static_lib])
    cleanup([c_file, c_object, static_lib])


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
    command = ["cargo", "rustc", "--target", target_triple]

    if config.arch == "leon":
        generate_leon_specification(config)
        command += ["--", "-C", "link-dead-code"]

    libname = "lib" + crate_name + ".a"
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
            return line.split("=", 1)[1].replace("\"", "").strip()

    print("Cargo.toml does not contain a project name")
    sys.exit(1)


def generate_c_dummy() -> str:
    """
    Writes a dummy C file that acts as 'glue' to get rust working with
    OctoPOS
    :return: The path to the dummy file
    """
    destination = "dummy.c"
    while os.path.isfile(destination):
        destination = "_" + destination

    data = "#include <stdint.h>\n" \
           "#include <octopos.h>\n" \
           "void rust_main_ilet(uint8_t claim_t);\n" \
           "void main_ilet(uint8_t claim_t) { rust_main_ilet(claim_t); }"

    with open(destination, 'w') as dummy:
        dummy.write(data)

    return destination
