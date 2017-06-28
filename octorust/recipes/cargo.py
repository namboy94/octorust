import shutil
from octorust.util.config import Config


def compile_using_cargo(config: Config):
    pass


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

def compile_cargo(config: Config):
    """
    Compiles a rust crate project using cargo and automatically
    includes octolib
    :param config: The compilation configuration
    :return: None
    """

    crate_name = os.path.basename(config.source)

    current = os.getcwd()
    os.chdir(config.source)

    command = ["cargo"]
    output = ""

    if config.arch == "x86guest":
        command += ["build", "--target", "i686-unknown-linux-gnu"]
        output = "target/i686-unknown-linux-gnu/debug/lib" + crate_name + ".a"
    elif config.arch == "x64native":
        command += ["build"]
        output = "target/debug/lib" + crate_name + ".a"
    elif config.arch == "leon":
        generate_leon_specification(config)
        command += ["rustc", "--target", "leon", "--", "-C", "link-dead-code"]
        output = "target/leon/debug/lib" + crate_name + ".a"

    print(command)
    generate_cargo_toml(config)
    Popen(command).wait()

    if not os.path.isfile(output):
        print("Compilation failed")
        sys.exit(1)

    os.rename(output, os.path.join(current, "lib" + crate_name + ".a"))
    cleanup(["leon.json", "Cargo.toml"])

    os.chdir(current)


