#!/usr/bin/env python3

import os
import time
import shutil
import argparse
from subprocess import Popen


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


def print_stats(title: str, stats: dict,
                lang_length: int = 22, value_length: int = 10,
                diff_length: int = 6, float_accuracy: int = 4,
                subtract_stats: dict = None):
    """
    Prints the benchmark statistics for a set of stats
    :param title: The Title of this section of stats (e.g. Runtimes)
    :param stats: The Stats themselves,
                  which are a language-indexed dict of values
    :param lang_length: The print length of the language
    :param value_length: The print length of the value
    :param diff_length: The print length of the difference between the value
                        and the minimum value
    :param float_accuracy: The printed accuracy of floating point values
    :param subtract_stats: A different set of stats that will be subtracted
                           from the other set of stats. Can be used to
                           avoid overemphasizing startup times for example
    :return: None
    """

    print("\033[1;34m" + title + "\033[0;0m:")

    minimum_stat = stats[min(stats, key=stats.get)]
    if subtract_stats is not None:
        minimum_stat -= subtract_stats[min(stats, key=stats.get)]

    for language in sorted(stats, key=stats.get):

        stat = stats[language]
        if subtract_stats is not None:
            stat -= subtract_stats[language]

        diff_stat = stat - minimum_stat

        # noinspection PyTypeChecker
        language_string = {
            "Rust": "\033[1;33m",
            "Rust (Debug)": "\033[1;32m",
            "Rust (Release)": "\033[1;33m",
            "C": "\033[1;31m",
            "X10": "\033[1;35m"
        }[language] + language + "\033[0;0m"

        if isinstance(stat, int):
            stat_string = str(stat)
            diff_string = str(diff_stat)

        else:
            stat_string = str(round(stat, float_accuracy))\
                .ljust(float_accuracy + 2, "0")
            diff_string = str(round(diff_stat, float_accuracy))\
                .ljust(float_accuracy + 2, "0")

        print(
            language_string.ljust(lang_length) +
            stat_string.ljust(value_length) +
            (" (+" + diff_string + ")").ljust(diff_length)
        )
    print()


def compile_with_octorust(path: str, language: str,
                          avoid_recompile: bool = False,
                          release: bool = False,
                          recompile_rust: bool = False) -> tuple:
    """
    Compiles a C or Rust Program using octorust
    :param path: The path to the C or Rust Program
    :param language: The language of the Program
    :param avoid_recompile: Avoids recompiling the program
                            if an executable already exists
    :param release: Enables optimizations in rust code
    :param recompile_rust: Recompiles rust dependencies
    :return: A tuple consisting of: 
                 The time it took to compile the program, 
                 The path to the compiled executable file
    """

    outfile = path + language + ".out"

    if os.path.isfile(outfile) and avoid_recompile:
        return 0.0, outfile

    if language.startswith("Rust") and recompile_rust:
        target_dir = os.path.join(path, "target")
        if os.path.isdir(target_dir):
            shutil.rmtree(target_dir)

    command = [
        "octorust",
        "-i", "2017-06-07",
        "-o", outfile,
        path
    ]

    if release:
        command.append("--release")

    compile_time = time.time()
    Popen(command).wait()
    return time.time() - compile_time, outfile


def compile_with_x10firm(path: str, avoid_recompile: bool = False) -> tuple:
    """
    Compiles an X10 program using x10firm
    :param path: The path to the file to compile
    :param avoid_recompile: Avoids recompiling the program
                            if an executable already exists
    :return: A tuple consisting of: 
                 The time it took to compile the program, 
                 The path to the compiled executable file
    """

    outfile = path + ".out"

    if os.path.isfile(outfile) and avoid_recompile:
        return 0.0, outfile

    compile_time = time.time()
    Popen([
        "x10firm",
        "-mtarget=i686-invasic-irtss",
        path, "-o", outfile
    ]).wait()
    return time.time() - compile_time, outfile


def run_benchmark(path: str, passes: int, use_median: bool = False) -> float:
    """
    Executes a file a given number of times and calculates the average
    runtime
    :param path: The path to the executable file
    :param passes: The amount of times the program should be executed
    :param use_median: Use the median instead of the mean
    :return: The mean or median runtime of the program
    """
    measurements = []
    for x in range(0, passes):
        start_time = time.time()
        Popen([path]).wait()
        measurements.append(time.time() - start_time)

    if use_median:
        return measurements[int(passes / 2)]
    else:
        return sum(measurements) / passes
    

# noinspection PyUnresolvedReferences
def run_benchmark_collection(benchmark_path: str, args: argparse.Namespace):
    """
    Runs all benchmarks found in the provided directory
    :param benchmark_path: The directory in which the individual
                           benchmarks are located
    :param args: The CLI arguments
    :return: A dictionary of runtimes, compile_times and file_sizes for each
             individual benchmark
    """

    compile_times = {}
    runtimes = {}
    file_sizes = {}

    for program in os.listdir(benchmark_path):
        program_path = os.path.join(benchmark_path, program)

        cargo_toml = os.path.join(program_path, "OctoCargo.toml")

        if os.path.isdir(program_path) and os.path.isfile(cargo_toml):

            if args.dual_rust_build:
                for variant, optimized in {
                    "Rust (Debug)": False, "Rust (Release)": True
                }.items():
                    print(variant)
                    compile_time, executable = compile_with_octorust(
                        program_path,
                        variant,
                        avoid_recompile=args.avoid_recompile,
                        release=optimized,
                        recompile_rust=args.recompile_rust
                    )
                    compile_times[variant] = compile_time
                    runtimes[variant] = run_benchmark(
                        executable, args.passes, args.use_median)
                    file_sizes[variant] = os.path.getsize(executable)
                    if not args.keep_executables:
                        os.remove(executable)
                continue

            else:
                language = "Rust"
                print(language)
                compile_time, executable = compile_with_octorust(
                    program_path,
                    language,
                    avoid_recompile=args.avoid_recompile,
                    release=True,
                    recompile_rust=args.recompile_rust
                )

        elif os.path.isfile(program_path) and program.endswith(".c"):
            language = "C"
            print(language)
            compile_time, executable = compile_with_octorust(
                program_path,
                language,
                avoid_recompile=args.avoid_recompile
            )

        elif os.path.isfile(program_path) and program.endswith(".x10"):

            if args.disable_x10:
                continue

            language = "X10"
            print(language)
            compile_time, executable = compile_with_x10firm(
                program_path,
                avoid_recompile=args.avoid_recompile
            )

        else:
            continue

        compile_times[language] = compile_time
        runtimes[language] = \
            run_benchmark(executable, args.passes, args.use_median)
        file_sizes[language] = os.path.getsize(executable)

        if not args.keep_executables:
            os.remove(executable)
        
    return {
        "compile_times": compile_times,
        "runtimes": runtimes,
        "file_sizes": file_sizes
    }


# noinspection PyUnresolvedReferences
def main():
    """
    Runs the benchmarks
    :return: None
    """

    # Magic variable that always points to the directory this file resides in
    directory = os.path.dirname(os.path.abspath(__file__))

    args = parse_args()
    benchmark_data = {}

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
            benchmark_data[benchmark] = \
                run_benchmark_collection(benchmark_path, args)

    print("\n")
    for benchmark, data in benchmark_data.items():

        runtimes = data["runtimes"]
        file_sizes = data["file_sizes"]
        compile_times = data["compile_times"]
        startup_runtimes = benchmark_data["startup"]["runtimes"]

        lang_length = 22 if "Rust (Debug)" not in runtimes else 30

        print("\033[1;36m" + benchmark + "\033[0;0m:\n")
        print_stats("Compile Times", compile_times, lang_length=lang_length)
        print_stats("Runtimes", runtimes, lang_length=lang_length)
        print_stats("Runtimes excluding startup", runtimes,
                    subtract_stats=startup_runtimes, lang_length=lang_length)
        print_stats("File Sizes", file_sizes, lang_length=lang_length)
        print("".ljust(38, "-"))


if __name__ == "__main__":
    main()
