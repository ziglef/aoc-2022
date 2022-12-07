#!/usr/bin/python3
import csv
import os
import re
import subprocess
import sys

RESULTS_FILE = "results.csv"


def compile_day_x(x: int) -> bool:
    path = "./d0" + str(x) if x < 10 else "./d" + str(x)
    try:
        os.chdir(path)
    except:
        print("Failed to find folder: {}".format(path))
        return False

    if os.system("cargo build --release") != 0:
        print("Failed to compile day: {}".format(x))
        os.chdir("..")
        return False
    
    os.chdir("..")


def run_day_x(x: int) -> str:
    path = "./d0" + str(x) + "/target/release/d0" + str(x) if x < 10 else "./d" + str(x) + "/target/release/d" + str(x)
    try:
        return str(subprocess.check_output(path, shell=True).decode('utf-8'))
    except subprocess.CalledProcessError:
        print("Failed to execute day: {}".format(x))
        raise


def handle_day_x(x: int) -> bool:
    print("Compiling day {}...".format(x))
    compile_day_x(x)

    print("Running day {}...".format(x))
    try:
        day_out = run_day_x(x)
    except:
        print("Failed to run day {}".format(x))
        return False

    print("Updating results for day {}...".format(x))
    with open(RESULTS_FILE) as results_file:
        with open(RESULTS_FILE + ".tmp", "w") as results_file_new:
            csv_reader = csv.DictReader(results_file)
            csv_writer = csv.DictWriter(results_file_new, fieldnames=csv_reader.fieldnames)

            csv_writer.writeheader()

            for row in csv_reader:
                if int(row["Day"]) == x:
                    row["Time input (ns)"] = re.search("(?<=Time input:\s)\d*", day_out).group(0)
                    row["Time input (%)"] = re.findall("\d+\.\d+", day_out)[0]
                    row["Time A (ns)"] = re.search("(?<=Time A:\s)\d*", day_out).group(0)
                    row["Time A (%)"] = re.findall("\d+\.\d+", day_out)[1]
                    row["Time B (ns)"] = re.search("(?<=Time B:\s)\d*", day_out).group(0)
                    row["Time B (%)"] = re.findall("\d+\.\d+", day_out)[2]
                    row["Time total"] = re.search("(?<=Time total:\s).*", day_out).group(0)
                csv_writer.writerow(row)
    
    print("Wrapping up day {}...".format(x))
    os.system("mv results.csv.tmp results.csv")

if __name__ == "__main__":
    if len(sys.argv) == 2:
        day = int(sys.argv[1])

        handle_day_x(day)
    elif len(sys.argv) == 1:
        for day in range(0, 26):
            handle_day_x(day)
    else:
        print("Usage:\tpython stats_helper.py <day_to_process>")
        print("")
        print("If you dont specify a day the script runs for all days.")