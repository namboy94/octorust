#!/usr/bin/env python3

import os
import argparse
from subprocess import Popen, check_output


def parse_args() -> argparse.Namespace:
    """
    Parses the arguments for the CLI
    :return: The parsed arguments
    """

    parser = argparse.ArgumentParser()
    parser.add_argument("-p", "--passes", type=int, default=1,
                        help="Sets the amount of time to run each program")
    parser.add_argument("--disable-x10", action="store_true",
                        help="Disables compiling X10 programs")
    parser.add_argument("--disable-c", action="store_true",
                        help="Disables compiling C programs")
    parser.add_argument("--disable-rust", action="store_true",
                        help="Disables compiling Rust programs")
    parser.add_argument("--recompile-rust", action="store_true",
                        help="Recompiles Rust dependencies every time")
    parser.add_argument("-b", "--blacklist",
                        help="A comma-seperated list of programs to skip")
    parser.add_argument("--use-median", action="store_true",
                        help="Uses the median instead of the mean when"
                             "averaging runtimes")
    parser.add_argument("--dual-rust-build", action="store_true",
                        help="Compiles both a debug and a release version of "
                             "rust programs")
    parser.add_argument("-k", "--keep-executables", action="store_true",
                        help="Stops the program from deleting the executable"
                             "files after executing them")
    parser.add_argument("--avoid-recompile", action="store_true",
                        help="Uses previously compiled executable files"
                             "if available")
    return parser.parse_args()


def compile_with_octorust(path: str, language: str,
                          avoid_recompile: bool = False) -> tuple:
    """
    Compiles a C or Rust Program using octorust
    :param path: The path to the C or Rust Program
    :param language: The language of the Program
    :param avoid_recompile: Avoids recompiling the program
                            if an executable already exists
    :return: A tuple consisting of:
                 The path to the compiled executable file,
                 The path to the optimized compiled executable file
    """
    proj_name = os.path.basename(os.path.dirname(path))
    outfile = proj_name + "-" + language
    outfile_opt = proj_name + "-opt"

    if os.path.isfile(outfile) and os.path.isfile(outfile_opt) and \
            avoid_recompile:
        return outfile, outfile_opt

    command = [
        "octorust",
        "-i", "2017-06-07",
        path
    ]

    Popen(command + ["-o", outfile]).wait()
    Popen(command + ["-o", outfile_opt, "--release"]).wait()
    return outfile, outfile_opt


def compile_with_x10firm(path: str, language: str,
                         avoid_recompile: bool = False) -> tuple:
    """
    Compiles an X10 program using x10firm
    :param path: The path to the file to compile
    :param language: The language, should always be X10
    :param avoid_recompile: Avoids recompiling the program
                            if an executable already exists
    :return: A tuple consisting of: 
                 The path to the compiled executable file,
                 The path to the optimized compiled executable file
    """
    proj_name = os.path.basename(os.path.dirname(path))
    outfile = proj_name + "-" + language
    outfile_opt = proj_name + "-opt"

    if os.path.isfile(outfile) and avoid_recompile:
        return outfile, outfile_opt

    command = [
        "x10firm",
        "-mtarget=1686-invasic-irtss",
        path
    ]

    Popen(command + ["-o", outfile]).wait()
    Popen(command + ["-o", outfile_opt, "-O3"]).wait()

    return outfile, outfile_opt
    

# noinspection PyUnresolvedReferences
def run_benchmark_collection(benchmark_path: str, args: argparse.Namespace):
    """
    Runs all benchmarks found in the provided directory
    :param benchmark_path: The directory in which the individual
                           benchmarks are located
    :param args: The CLI arguments
    :return: None
    """
    for program in os.listdir(benchmark_path):
        program_path = os.path.join(benchmark_path, program)

        cargo_toml = os.path.join(program_path, "Cargo.toml")

        if os.path.isdir(program_path) and os.path.isfile(cargo_toml):
            if args.disable_rust:
                continue
            language = "Rust"
            compiler_fn = compile_with_octorust

        elif os.path.isfile(program_path) and program.endswith(".c"):
            if args.disable_c:
                continue
            language = "C"
            compiler_fn = compile_with_octorust

        elif os.path.isfile(program_path) and program.endswith(".x10"):
            if args.disable_x10:
                continue
            language = "X10"
            compiler_fn = compile_with_x10firm

        else:
            continue

        executable, opt_executable = \
            compiler_fn(program_path, language, args.avoid_recompile)

        run_temci(executable, args.passes)
        run_temci(opt_executable, args.passes)

        if not args.keep_executables:
            os.remove(executable)
            os.remove(opt_executable)


def run_temci(executable: str, runs: int):

    if not os.path.isdir("temci_output"):
        os.makedirs("temci_output")

    exec_name = os.path.basename(executable)
    output_yaml_file = os.path.join("temci_output", exec_name + ".yaml")
    output_txt_file = os.path.join("temci_output", exec_name + ".txt")

    printed = check_output(
        ["temci", "short", "exec", "-wd", executable, "--runs", str(runs),
         "--out", output_yaml_file]
    ).decode("utf-8")
    with open(output_txt_file, 'w') as f:
        f.write(printed)


# noinspection PyUnresolvedReferences
def main():
    """
    Runs the benchmarks
    :return: None
    """

    # Magic variable that always points to the directory this file resides in
    directory = os.path.dirname(os.path.abspath(__file__))

    args = parse_args()

    benchmarks = [
        "startup",
        "primes-naive",
        "primes-eratosthenes",
        "garbageonly-gc-benchmark"
    ]

    if args.blacklist is not None:
        for blacklist_element in args.blacklist.split(","):
            if blacklist_element in benchmarks:
                benchmarks.remove(blacklist_element)

    for benchmark in benchmarks:
        print("Benchmark " + benchmark)
        benchmark_path = os.path.join(directory, benchmark)

        if not os.path.isdir(benchmark_path):
            continue
        else:
            run_benchmark_collection(benchmark_path, args)

    print("Done")

if __name__ == "__main__":
    main()
