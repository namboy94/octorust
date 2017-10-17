#!/usr/bin/env python3

import os
import time
import shutil
import argparse
from subprocess import Popen

# Helper constants
octorust_base_cmd = [
    "octorust",
    "-i", "2017-06-07",
    "-a", "x86guest",
    "-v", "generic",
    "-o", "out",
    "--release"
]
directory = os.path.dirname(os.path.abspath(__file__))


def colour_lang(l):

    colour = {
        "Rust": "\033[1;33m",
        "C": "\033[1;31m",
        "X10": "\033[1;35m"
    }[l]
    return colour + l + "\033[0;0m"


def parse_args():

    parser = argparse.ArgumentParser()
    parser.add_argument("-p", "--passes", type=int,
                        help="Sets the amount of time to run each program")
    parser.add_argument("--disable-x10", action="store_true",
                        help="Disables compiling X10 programs")
    parser.add_argument("--recompile-rust", action="store_true",
                        help="Recompiles Rust dependencies every time")
    return parser.parse_args()


if __name__ == "__main__":

    # change-able constants
    RUN_COUNT = 1
    DISABLE_X10 = False
    RECOMPILE_RUST = False

    args = parse_args()

    if args.passes is not None:
        RUN_COUNT = args.passes
    if args.disable_x10 is not None:
        DISABLE_X10 = args.disable_x10
    if args.recompile_rust is not None:
        RECOMPILE_RUST = args.recompile_rust

    bench_data = {}

    for eval_dir in [
        "startup",
        "primes-naive",
        "primes-eratosthenes",
        "garbageonly-gc-benchmark"
    ]:
        eval_dir_path = os.path.join(directory, eval_dir)

        runtimes = {}
        sizes = {}
        compile_times = {}

        if os.path.isdir(eval_dir_path):

            for test in os.listdir(eval_dir_path):
                test_file = os.path.join(eval_dir_path, test)

                lang = "generic"

                if test.endswith(".c") or os.path.isdir(test_file):

                    if os.path.isdir(test_file) and RECOMPILE_RUST:
                        # Remove pre-compiled stuff to make compile time
                        # comparisons fair
                        target_dir = os.path.join(test_file, "target")
                        if os.path.isdir(target_dir):
                            shutil.rmtree(target_dir)

                    lang = "C" if test.endswith(".c") else "Rust"
                    compile_start_time = time.time()
                    Popen(octorust_base_cmd + [test_file]).wait()
                    compile_times[lang] = time.time() - compile_start_time

                elif test.endswith(".x10"):

                    if DISABLE_X10:
                        continue

                    lang = "X10"
                    compile_start_time = time.time()
                    Popen([
                        "x10firm", "-v",
                        "-mtarget=i686-invasic-irtss",
                        test_file, "-o", "out"
                    ]).wait()
                    compile_times[lang] = time.time() - compile_start_time

                else:
                    continue

                times = []
                for x in range(0, RUN_COUNT + 1):
                    start_time = time.time()
                    Popen(["./out"]).wait()
                    end_time = time.time()
                    times.append(end_time - start_time)

                runtimes[lang] = sum(times) / RUN_COUNT
                sizes[lang] = os.path.getsize("out")
                os.remove("out")

        bench_data[eval_dir] = (runtimes, sizes, compile_times)

    print("\n")
    for data in bench_data.keys():
        runtimes = bench_data[data][0]
        sizes = bench_data[data][1]
        compile_times = bench_data[data][2]

        min_compile_time = \
            compile_times[min(compile_times, key=compile_times.get)]
        min_runtime = runtimes[min(runtimes, key=runtimes.get)]
        min_filesize = sizes[min(sizes, key=sizes.get)]

        print("\033[1;36m" + data + "\033[0;0m:\n")

        print("\033[1;34mCompile Times\033[0;0m:")
        for lang in sorted(compile_times, key=compile_times.get):
            compile_time = compile_times[lang]
            print(colour_lang(lang).ljust(22) +
                  str(round(compile_time, 4)).ljust(10) +
                  " (+" + str(round(compile_time - min_compile_time, 4)) + ")")

        print("\033[1;34mRuntimes\033[0;0m:")
        for lang in sorted(runtimes, key=runtimes.get):
            runtime = runtimes[lang]
            print(colour_lang(lang).ljust(22) +
                  str(round(runtime, 4)).ljust(10) +
                  " (+" + str(round(runtime - min_runtime, 4)) + ")")

        print("\033[1;34mFile Sizes\033[0;0m:")
        for lang in sorted(sizes, key=sizes.get):
            filesize = sizes[lang]
            print(colour_lang(lang).ljust(22) +
                  str(filesize).ljust(10) +
                  " (+" + str(filesize - min_filesize) + ")")

        print("-------------------------------------------------------------")
