#!/usr/bin/env python3

import json

import matplotlib as mpl
import matplotlib.font_manager as fm
import matplotlib.pyplot as plt


def main():
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

    xs = []
    ys = []

    for name, rose_data in rose.items():
        seconds = rose_data.get("seconds")
        tfjs_data = tfjs.get(name)
        if seconds and tfjs_data:
            rose_time = seconds["optimizing"]
            tfjs_time = tfjs_data["seconds"]["optimizing"]
            xs.append(tfjs_time)
            ys.append(rose_time)

    for font in fm.findSystemFonts(["fonts"]):
        fm.fontManager.addfont(font)
    mpl.rcParams["font.family"] = "Linux Libertine"
    mpl.rcParams["font.size"] = 10

    # https://pldi24.sigplan.org/#formatting-requirements
    plt.figure(figsize=(5.478, 4))
    plt.xscale("log")
    plt.yscale("log")
    plt.scatter(xs, ys, color="black", alpha=0.5)
    plt.xlabel("TensorFlow.js (seconds)")
    plt.ylabel("Rose (seconds)")
    plt.savefig("scatter.pdf")


if __name__ == "__main__":
    main()
