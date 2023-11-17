#!/usr/bin/env python3

import json

import matplotlib as mpl
import matplotlib.font_manager as fm
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns


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

    # Read the TFJS compile time data
    with open("e7ecab9/data.json", "r") as file:
        tfjs_compiles = json.load(file)

    with open("0f1efff/data.json", "r") as file:
        tfjs_node = json.load(file)

    with open("654e94c/data.json", "r") as file:
        tfjs_node_gpu = json.load(file)

    # Read the Rose data
    with open("rose/data.json", "r") as file:
        rose = json.load(file)

    xs = []
    ys = []
    ratios = []

    print("omitted:")
    for name, rose_data in rose.items():
        seconds = rose_data.get("seconds")
        if seconds:
            tfjs_data = tfjs.get(name)
            if tfjs_data:
                tfjs_delta = tfjs_compiles[name]["seconds"]
                rose_time = seconds["autodiff"] + seconds["optimizing"]
                tfjs_time = tfjs_delta["autodiff"] + tfjs_data["seconds"]["optimizing"]
                ratio = tfjs_time / rose_time
                xs.append(tfjs_time)
                ys.append(rose_time)
                ratios.append(ratio)
            else:
                print("failure ", name)
        else:
            print("non-trio", name)

    print()
    print("ratios:")
    for p in range(25, 100, 25):
        print(f"{p:2d}% {np.percentile(ratios, p)}")

    node_ratios = []
    gpu_ratios = []

    for name, tfjs_data in tfjs.items():
        node = tfjs_node.get(name)
        gpu = tfjs_node_gpu.get(name)
        seconds = tfjs_data.get("seconds")
        if seconds and node and gpu:
            node_ratios.append(node["seconds"]["optimizing"] / seconds["optimizing"])
            gpu_ratios.append(gpu["seconds"]["optimizing"] / seconds["optimizing"])

    print()
    print(f"node: {np.median(node_ratios)}")
    print(f"gpu: {np.median(gpu_ratios)}")

    for font in fm.findSystemFonts(["fonts"]):
        fm.fontManager.addfont(font)
    mpl.rcParams["font.family"] = "Linux Libertine"
    mpl.rcParams["font.size"] = 10

    # https://pldi24.sigplan.org/#formatting-requirements
    fig, axs = plt.subplots(1, 2, layout="constrained", figsize=(5.478, 2.5))

    axs[0].set_xscale("log")
    axs[0].set_yscale("log")
    axs[0].scatter(xs, ys, color="black", alpha=0.5)
    axs[0].set_xlabel("TensorFlow.js (seconds)")
    axs[0].set_ylabel("Rose (seconds)")

    sns.kdeplot(ratios, ax=axs[1], log_scale=True, fill=True, color="black")
    axs[1].set_xlabel("TensorFlow.js / Rose")
    axs[1].set_ylabel("probability density")

    fig.savefig("plots.pdf")


if __name__ == "__main__":
    main()
