#!/usr/bin/env python3
import os
import sys
import yaml
import math
import argparse


def parse_args() -> str:

    parser = argparse.ArgumentParser()
    parser.add_argument("temci_output", action="store", nargs="+",
                        help="The temci output YAML file(s)")
    parser.add_argument("--speedup", action="store_true")
    args = parser.parse_args()

    if args.speedup:
    	print_speedup()
    	sys.exit(0)

    return args.temci_output


def print_speedup():

	for lang in ["C", "Rust", "X10"]:
		for variant in ["", "-opt"]:
			with open("temci_output/primes-parallel-one-" + lang + variant + ".yaml", 'r') as f:
				one = yaml.load(f)[0]["data"]["etime"]
			with open("temci_output/primes-parallel-eight-" + lang + variant + ".yaml", 'r') as f:
				eight = yaml.load(f)[0]["data"]["etime"]
			
			one_mean = calculate_mean(one)
			one_median = calculate_median(one)
			eight_mean = calculate_mean(eight)
			eight_median = calculate_median(eight)

			mean_speedup = one_mean / eight_mean
			median_speedup = one_median / eight_median
			mean_eff = mean_speedup / 8
			median_eff = median_speedup / 8

			print(lang + variant)
			print(
				str(round(mean_speedup, 4)) + " & " + str(round(median_speedup, 4)) + " & " + str(round(mean_eff, 4)) + " & " + str(round(median_eff, 4))
			)
			print()


def main():
    for temci_yaml in parse_args():

        txt_file = temci_yaml.rsplit(".", 1)[0] + ".txt"
        if not os.path.isfile(temci_yaml) or not temci_yaml.endswith(".yaml"):
            continue

        print("\033[1;33m" + temci_yaml + "\033[0;0m")

        with open(temci_yaml, 'r') as f:
            temci_yaml = yaml.load(f)

        e_times = temci_yaml[0]["data"]["etime"]

        mean = round(calculate_mean(e_times), 4)
        median = round(calculate_median(e_times), 4)
        std_dev = round(calculate_standard_deviation(e_times), 4)
        variation_coefficient = round(calculate_variation_coefficient(e_times, True), 4)

        print("\033[1;31m", end="")
        print(
            str(mean).ljust(6, "0") + " & " +
            str(median).ljust(6, "0") + " & " +
            str(std_dev).ljust(6, "0") + " & " +
            str(variation_coefficient).ljust(6, "0") + "\\% \033[0;0m"
        )
        print()


def calculate_mean(data: list) -> float:
    return sum(data) / len(data)


def calculate_median(data: list) -> float:
    if len(data) % 2 == 0:
        lower_median = sorted(data)[int(len(data) / 2) - 1]
        upper_median = sorted(data)[int(len(data) / 2)]
        return (lower_median + upper_median) / 2
    else:
        return sorted(data)[int(len(data) / 2)]


def calculate_standard_deviation(data: list) -> float:

    average = calculate_mean(data)
    new = list(map(lambda x: (x - average) ** 2, data))
    return math.sqrt(sum(new) / (len(new) - 1))


def calculate_variation_coefficient(data: list, percentage: bool = False) -> float:

    std_dev = calculate_standard_deviation(data)
    var_co = std_dev / calculate_mean(data)
    return var_co if not percentage else var_co * 100


if __name__ == "__main__":
    main()
