#!/usr/bin/env python

import os
import sys
import time
from subprocess import Popen

# constants
RUN_COUNT = 1 if len(sys.argv) == 1 else int(sys.argv[1])


if __name__ == "__main__":

    octorust_base_cmd = [
        "octorust",
        "-i", "2017-06-07",
        "-a", "x86guest",
        "-v", "generic",
        "-o", "out"
    ]

    directory = os.path.dirname(os.path.abspath(__file__))

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

        if os.path.isdir(eval_dir_path):

            for test in os.listdir(eval_dir_path):
                test_file = os.path.join(eval_dir_path, test)

                lang = "generic"

                if test.endswith(".c") or os.path.isdir(test_file):
                    lang = "C" if test.endswith(".c") else "Rust"
                    Popen(octorust_base_cmd + [test_file]).wait()

                elif test.endswith(".x10"):
                    lang = "X10"
                    print("TODO: X10")
                    continue

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

        bench_data[eval_dir] = (runtimes, sizes)

    for data in bench_data.keys():
        runtimes = bench_data[data][0]
        sizes = bench_data[data][1]

        print("\033[1;36m" + data + "\033[0;0m:\n")

        print("\033[1;34mRuntimes\033[0;0m:")
        for lang in sorted(runtimes, key=runtimes.get):
            runtime = runtimes[lang]
            # noinspection PyUnresolvedReferences
            print(lang.ljust(15) + str(round(runtime, 4)))

        print("\033[1;34mFile Sizes\033[0;0m:")
        for lang in sorted(sizes, key=sizes.get):
            filesize = sizes[lang]
            # noinspection PyUnresolvedReferences
            print(lang.ljust(15) + str(filesize))

        print("--------------------------------------------------------------")


