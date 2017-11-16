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
import sys
import toml
from subprocess import Popen
from octorust.util.files import cleanup
from octorust.util.config import Config
from octorust.linking.octopos_link import link_app
from octorust.recipes.c import compile_c_object
from octorust.recipes.rust import get_rust_target_triple,\
    generate_leon_specification


def compile_using_cargo(config: Config):
    """
    Compiles a rust crate using cargo.
    :param config: The configuration to use while compiling
    :return: None
    """

    static_lib = compile_static_library(config)
    c_file = generate_c_dummy()
    c_object = compile_c_object(config, c_file)
    link_app(config, [c_object, static_lib])
    cleanup([c_file, c_object, static_lib], config)


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

    validate_cargo_toml_file()
    old_cargo_toml = inject_octolib_into_cargo_toml(config)

    try:
        target_triple = get_rust_target_triple(config.arch)
        command = ["cargo", "rustc"]
        if config.release:
            command.append("--release")
        command += ["--target", target_triple]

        if config.arch == "leon":
            generate_leon_specification(config.gcc)
            command += ["--", "-C", "link-dead-code"]

        crate_name = read_crate_name_from_cargo_toml("Cargo.toml")
        libname = "lib" + crate_name.replace("-", "_") + ".a"
        output_name = "release" if config.release else "debug"
        output = os.path.join("target", target_triple, output_name, libname)

        print(command)
        Popen(command).wait()

        if not os.path.isfile(output):
            print("Failed to compile the static rust library")
            sys.exit(1)

        os.rename(output, os.path.join(current, libname))
        cleanup(["leon.json"], config)

        restore_cargo_toml(old_cargo_toml)
        os.chdir(current)
        return libname

    except BaseException as e:
        restore_cargo_toml(old_cargo_toml)
        raise e


def validate_cargo_toml_file():
    """
    Makes sure that the Cargo.toml file is valid
    :return: None
    """
    if not os.path.isfile("Cargo.toml"):
        print("Missing Cargo.toml file")
        sys.exit(1)

    with open("Cargo.toml", 'r') as cargo_toml_file:
        try:
            cargo_toml = toml.loads(cargo_toml_file.read())
        except toml.TomlDecodeError:
            print("The Cargo.toml file is invalid")
            sys.exit(1)

    if "lib" not in cargo_toml \
            or "crate-type" not in cargo_toml["lib"] \
            or cargo_toml["lib"]["crate-type"] != ["staticlib"]:
        print("The project's crate type is not set to ['staticlib']")
        print("Please change the crate type to ['staticlib']")
        sys.exit(1)


def inject_octolib_into_cargo_toml(config: Config) -> toml:
    """
    Injects a dependency for the octolib crate into the project's
    Cargo.toml file
    :param config: The config to use while compiling
    :return: A backup of the Cargo.toml content
    """

    with open("Cargo.toml", 'r') as cargo_toml_file:
        cargo_toml_content = cargo_toml_file.read()
        cargo_toml = toml.loads(cargo_toml_content)
        cargo_toml_backup = toml.loads(cargo_toml_content)

    if "dependencies" not in cargo_toml:
        cargo_toml["dependencies"] = {}
    if "octolib" not in cargo_toml["dependencies"]:
        cargo_toml["dependencies"]["octolib"] = {}

    octolib_version = get_octolib_version(config)
    cargo_toml["dependencies"]["octolib"]["path"] = config.octolib
    cargo_toml["dependencies"]["octolib"]["version"] = octolib_version

    with open("Cargo.toml", 'w') as cargo_toml_file:
        cargo_toml_file.write(toml.dumps(cargo_toml))

    return cargo_toml_backup


def restore_cargo_toml(toml_backup: toml):
    """
    Writes the old Cargo.toml data back to the Cargo.toml file
    :param toml_backup: The previous Cargo.toml data
    :return: None
    """

    with open("Cargo.toml", "w") as f:
        f.write(toml.dumps(toml_backup))


def get_octolib_version(config: Config) -> str:
    """
    Retrieves the version of the currently installed octolib crate
    :param config: The configuration used when compiling
    :return: The version string of the octolib crate
    """
    with open(os.path.join(config.octolib, "Cargo.toml"), 'r') as f:
        cargo_toml = toml.loads(f.read())
        return cargo_toml["package"]["version"]


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

    # C Dummy file content
    data = "#include <stdint.h>\n" \
           "#include <octopos.h>\n" \
           "void rust_main_ilet(uint8_t claim_t);\n" \
           "void main_ilet(uint8_t claim_t) { \n" \
           "    rust_main_ilet(claim_t);\n" \
           "    shutdown(0);\n" \
           "}"

    with open(destination, 'w') as dummy:
        dummy.write(data)

    return destination
