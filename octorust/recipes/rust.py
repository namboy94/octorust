import json
from octorust.util.config import Config


def get_rust_target_triple(arch: str) -> str:
    """
    Figures out the target triple for an architecture
    identifier
    :param arch: The architecture for which the target
                 triple is wanted for
    :return: The target triple
    """
    return {
        "x86guest": "i686-unknown-linux-gnu",
        "x64native": "x86_64-unknown-linux-gnu",
        "leon": "leon"
    }[arch]


def generate_leon_specification(config: Config):
    """
    Generates a rust target specification file for the SPARC LEON architecture
    The specification file assumes a gcc cross-compiler for the architecture
    in the path called 'sparc-leon-linux-uclibc-gcc', which is one of the
    cargo_sample targets when using crosstool-ng.
    The installation process should in theory install a sparc-elf-gcc
    compiler though.
    The specification file is saved as leon.json
    :param config: The configuration used
    :return: None
    """

    leon_spec = {
        "arch": "sparc",
        "data-layout": "E-m:e-p:32:32-i64:64-f128:64-n32-S64",
        "executables": True,
        "llvm-target": "sparc",
        "os": "none",
        "panic-strategy": "abort",
        "target-endian": "big",
        "target-pointer-width": "32",
        "linker-flavor": "ld",
        "linker": config.gcc,
        "link-args": [
            "-nostartfiles"
        ]
    }

    with open("leon.json", 'w') as spec_file:
        json.dump(leon_spec, spec_file)
