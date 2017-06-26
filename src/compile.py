#!/usr/bin/env python3

# TODO Cleanup, Comments, Maybe splitting up into more source files

import os
import json
import argparse
from typing import Tuple
from subprocess import Popen


def parse_args() -> Tuple[str, str, str, str]:
    """
    Parses the arguments
    :return: A tuple consisting of: The target architecture, 
                                    The target variant,
                                    The input file location,
                                    The output file location
    """

    parser = argparse.ArgumentParser()
    parser.add_argument("architecture", help="The target architecture for the application. "
                                             "May be one of (leon|x86guest|x64native)")
    parser.add_argument("variant", help="Specifies the variant of the target, for example 'generic' "
                                        "or '4t5c-chipit' etc.")
    parser.add_argument("input", help="The input file")
    parser.add_argument("output", help="The output file")

    args = parser.parse_args()
    arch = args.architecture
    variant = args.variant
    source = args.input
    out = args.output

    return arch, variant, source, out


def main():

    arch, variant, source, out = parse_args()
    rustlib = compile_rust(arch, source)
    c_dummy = generate_c_dummy()
    c_object = compile_c_object(arch, variant, c_dummy)
    link_app(arch, variant, rustlib, c_object, out)


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


def compile_rust(arch: str, source: str) -> str:
    """
    Compiles the rust file to a static library for use in OctoPOS
    :param arch: The architecture for which to compile to 
    :param source: The source file to compile
    :return: The path to the generated library file
    """

    rustc = ["rustc"]

    if arch == "x86guest":
        rustc += ["--target", "i686-unknown-linux-gnu"]
    elif arch == "leon":
        generate_leon_specification()
        rustc += ["--target", "leon"]

    rustc += [source]
    Popen(rustc).wait()

    return "lib" + source.rsplit(".rs", 1)[0] + ".a"


def generate_leon_specification():

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
        "linker": "sparc-leon-linux-uclibc-gcc",
        "link-args": [
            "-nostartfiles"
        ]
    }

    with open("leon.json", 'w') as spec_file:
        json.dump(leon_spec, spec_file)




if __name__ == "__main__":
    main()