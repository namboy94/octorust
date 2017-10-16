#!/usr/bin/env python3

import os
import sys
import time
import shutil
from subprocess import Popen

# constants
RUN_COUNT = 1 if len(sys.argv) == 1 else int(sys.argv[1])
ENABLE_X10 = False
RECOMPILE_RUST = False


def colour_lang(l):

    colour = {
        "Rust": "\033[1;33m",
        "C": "\033[1;31m",
        "X10": "\033[1;35m"
    }[l]
    return colour + l + "\033[0;0m"

if __name__ == "__main__":

    octorust_base_cmd = [
        "octorust",
        "-i", "2017-06-07",
        "-a", "x86guest",
        "-v", "generic",
        "-o", "out",
        "--release"
    ]

    directory = os.path.dirname(os.path.abspath(__file__))

    bench_data = {}

    for eval_dir in [
        "startup",
        "primes-naive",
        # "primes-eratosthenes",
        # "garbageonly-gc-benchmark"
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

                    if not ENABLE_X10:
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

    for data in bench_data.keys():
        runtimes = bench_data[data][0]
        sizes = bench_data[data][1]
        compile_times = bench_data[data][2]

        print("\033[1;36m" + data + "\033[0;0m:\n")

        print("\033[1;34mCompile Times\033[0;0m:")
        for lang in sorted(compile_times, key=compile_times.get):
            compile_time = compile_times[lang]
            # noinspection PyUnresolvedReferences
            print(colour_lang(lang).ljust(22) + str(round(compile_time, 4)))

        print("\033[1;34mRuntimes\033[0;0m:")
        for lang in sorted(runtimes, key=runtimes.get):
            runtime = runtimes[lang]
            # noinspection PyUnresolvedReferences
            print(colour_lang(lang).ljust(22) + str(round(runtime, 4)))

        print("\033[1;34mFile Sizes\033[0;0m:")
        for lang in sorted(sizes, key=sizes.get):
            filesize = sizes[lang]
            # noinspection PyUnresolvedReferences
            print(colour_lang(lang).ljust(22) + str(filesize))

        print("-------------------------------------------------------------")
