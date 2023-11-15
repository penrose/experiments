#!/usr/bin/env python3

import csv
import json

# Read the exclusions from the JSON file
with open("exclude.json", "r") as file:
    exclusions = json.load(file)

tfjs = {}
for commit, excludes in exclusions.items():
    for i, exclude in enumerate(excludes):
        # Read data for each exclude set
        with open(f"{commit}/data{i}.json", "r") as file:
            datas = json.load(file)

        # Process the data
        for name, data in datas.items():
            if name not in exclude:
                if name in tfjs:
                    raise Exception(f"duplicate diagram: {name}")
                tfjs[name] = data

# Read the Rose data
with open("rose/data.json", "r") as file:
    rose = json.load(file)


def get_time(data):
    if data is None:
        return ""
    return data.get("seconds", {}).get("optimizing", "") or data.get("totalSeconds", "")


# Write to the CSV file using csv module
with open("data.csv", "w", newline="") as csvfile:
    csvwriter = csv.writer(csvfile)
    csvwriter.writerow(["name", "tfjs", "rose"])
    for name, data in rose.items():
        csvwriter.writerow([name, get_time(tfjs.get(name)), get_time(data)])
