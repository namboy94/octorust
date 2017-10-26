#!/usr/bin/env python

import os
import sys
import yaml
import math
import argparse


def parse_args() -> str:

    parser = argparse.ArgumentParser()
    parser.add_argument("temci_output", action="store", nargs="+",
                        help="The temci output YAML file(s)")
    args = parser.parse_args()
    return args.temci_output


def main():

    for temci_yaml in parse_args():

        txt_file = temci_yaml.rsplit(".", 1)[0] + ".txt"
        if not os.path.isfile(temci_yaml) or not temci_yaml.endswith(".yaml"):
            continue

        print("\033[1;33m" + temci_yaml + "\033[0;0m")

        with open(temci_yaml, 'r') as f:
            temci_yaml = yaml.load(f)

        with open(txt_file, 'r') as f:
            pass  # print(f.read())

        e_times = temci_yaml[0]["data"]["etime"]
        print("mean:                " + str(calculate_mean(e_times)))
        print("median:              " + str(calculate_median(e_times)))
        print("MAD:                 " + str(calculate_median_absolute_deviation(e_times)))
        print("mean max deviation:  " + str(calculate_maximum_absolute_deviation(e_times, True)))
        print("median max deviation:" + str(calculate_maximum_absolute_deviation(e_times)))
        print("mean std deviation:  " + str(calculate_standard_deviation(e_times, True)))
        print("median std deviation:" + str(calculate_standard_deviation(e_times)))


def calculate_mean(data: list) -> float:
    return sum(data) / len(data)


def calculate_median(data: list) -> float:
    if len(data) % 2 == 0:
        lower_median = sorted(data)[int(len(data) / 2) - 1]
        upper_median = sorted(data)[int(len(data) / 2)]
        return (lower_median + upper_median) / 2
    else:
        return sorted(data)[int(len(data) / 2)]


def calculate_median_absolute_deviation(data: list) -> float:
    median = calculate_median(data)
    new = list(map(lambda x: abs(x - median), data))
    return calculate_median(new)


def calculate_maximum_absolute_deviation(data: list, use_mean: bool = False) \
        -> float:

    average = calculate_mean(data) if use_mean else calculate_median(data)
    # noinspection PyTypeChecker
    return max(map(lambda x: abs(x - average), data))


def calculate_standard_deviation(data: list, use_mean: bool = False) -> float:

    average = calculate_mean(data) if use_mean else calculate_median(data)
    new = list(map(lambda x: (x - average) ** 2, data))
    return math.sqrt(calculate_mean(new) if use_mean else calculate_median(new))


if __name__ == "__main__":
    main()