import os
import sys
from subprocess import Popen
from octorust.util.config import Config
from octorust.util.files import cleanup
from octorust.linking.octopos_link import link_app
from octorust.recipes.rust import generate_leon_specification, \
    get_rust_target_triple


def compile_using_rustc(config: Config):
    """
    Compiles a single rust file
    :param config: The compilation configuration
    :return: None
    """

    if config.arch == "leon":
        generate_leon_specification(config)

    rust_object = compile_rust_object(config)
    link_app(config, [rust_object])

    cleanup(["leon.json", rust_object], config)


def compile_rust_object(config: Config) -> str:
    """
    Compiles an object file from a .rs file for the correct architecture
    :param config: The configuration to use while compiling
    :return: The path to the generated object file
    """

    object_file = config.output + ".o"

    command = ["rustc", "--emit", "obj", "-o", object_file, "--target",
               get_rust_target_triple(config.arch), config.source]

    print(command)
    Popen(command).wait()

    if not os.path.isfile(object_file):
        print("Could not compile rust object file.")
        sys.exit(1)

    return object_file
