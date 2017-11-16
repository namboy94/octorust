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

import json


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


def generate_leon_specification(sparc_elf_gcc_path):
    """
    Generates a rust target specification file for the SPARC LEON architecture
    The specification file assumes a gcc cross-compiler for the architecture
    in the path called 'sparc-leon-linux-uclibc-gcc', which is one of the
    cargo_sample targets when using crosstool-ng.
    The installation process should in theory install a sparc-elf-gcc
    compiler though.
    The specification file is saved as leon.json
    :param sparc_elf_gcc_path: The path to the SPARC ELF gcc compiler
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
        "linker": sparc_elf_gcc_path,
        "link-args": [
            "-nostartfiles"
        ]
    }

    with open("leon.json", 'w') as spec_file:
        json.dump(leon_spec, spec_file)
