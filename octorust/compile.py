#!/usr/bin/env python3
"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

# TODO Cleanup, Comments, Maybe splitting up into more source files

import os
import json
from subprocess import Popen
from octorust.cli import parse_args


def main():

    config = parse_args()

    rustlib = compile_rust(arch, source)
    c_dummy = generate_c_dummy()
    c_object = compile_c_object(arch, variant, c_dummy)
    link_app(arch, variant, rustlib, c_object, out)
    cleanup()


def generate_c_dummy() -> str:

    data = "#include <stdint.h>\n" \
           "#include <octopos.h>\n" \
           "void main_rust_ilet(uint8_t claim_t);\n" \
           "void main_ilet(uint8_t claim_t) {\n" \
           "    main_rust_ilet(claim_t);\n" \
           "}"

    with open("dummy.c", 'w') as dummy:
        dummy.write(data)

    return "dummy.c"


def get_gcc(arch: str) -> str:
    return {"x86guest": "gcc", "x64native": "gcc", "leon": "sparc-elf-gcc"}[arch]


def compile_c_object(arch: str, variant: str, c_file: str) -> str:

    command = [get_gcc(arch)]

    if arch == "x86guest":
        command.append("-mfpmath=sse")
        command.append("-msse2")
        command.append("-m32")  # 32-bit
        command.append("-nostdinc")
        command.append("-fno-asynchronous-unwind-tables")
        command.append("-fno-stack-protector")
        command.append("-I" + os.path.join(get_irtss_release_path(arch, variant), "include"))
        command.append("-isystem")
        command.append("/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/include")  # TODO make generic
        command.append("-D__OCTOPOS__")
        command.append("-std=gnu11")
    elif arch == "x64native":
        pass  # TODO
    elif arch == "leon":
        pass  # TODO

    object_file = c_file.rsplit(".c", 1)[0] + ".o"
    command += ["-c", c_file, object_file]

    Popen(command).wait()
    return object_file


def link_app(arch: str, variant: str, rustlib: str, c_object: str, out: str):

    irtss_path = get_irtss_release_path(arch, variant)
    command = [get_gcc(arch), "-o", out]

    " -static"

    if arch == "x86guest":
        command.append("-m32")  # 32 bit
        command.append("-L" + os.path.join(irtss_path, "lib"))
        command.append("-nostdlib")
        command.append("-Wl,-T," + os.path.join(irtss_path, "lib", "sections.x"))
        command.append(get_native_object("crti.o"))
        command.append(get_native_object("crtbegin.o"))
        command.append(c_object)
        command.append(rustlib)
        command.append("-loctopos")
        command.append("-lc++")
        command.append("-lcsubset")
        command.append("-loctopos")
        command.append("-lgcc")
        command.append(get_native_object("crtend.o"))
        command.append(get_native_object("crtn.o"))
        command.append("-lotail")
        command.append("-static")

    elif arch == "x64native":
        pass
    elif arch == "leon":
        pass

    Popen(command).wait()


def get_native_object(target: str) -> str:
    # TODO make this better
    if target == "crti.o":
        return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/../../../../lib32/crti.o"
    elif target == "crtbegin.o":
        return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/32/crtbegin.o"
    elif target == "crtend.o":
        return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/32/crtend.o"
    elif target == "crtn.o":
        return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/../../../../lib32/crtn.o"


def get_irtss_release_path(arch: str, variant: str) -> str:
    # TODO look for better solution
    return os.path.join("../..", "releases", "current", arch, variant)






def cleanup():
    cleanup_files = [
        "dummy.c",
    ]

    for element in cleanup_files:
        if os.path.isfile(element):
            os.remove(element)



if __name__ == "__main__":
    main()