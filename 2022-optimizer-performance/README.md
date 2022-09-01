# 2022 Optimizer Performance

author: [**@samestep**](http://github.com/samestep)

This experiment is an attempt to get an idea of how much faster the Penrose
optimizer could possibly be, without fundamentally changing how it works.
Specifically, I naïvely rewrote the optimizer in Rust (i.e. direct translation,
nothing clever) and modified our autodiff code generator to produce C code
instead of JavaScript. I also added a counter to the optimizer to see how many
times it called the generated gradient function. Then I timed both the original
JavaScript optimizer and the native optimizer (including the time it takes to
actually compile the Rust and generated C code) and compared them, in each case
separating out the total time it takes to call the generated gradient function
from the time spent in the rest of the optimizer.

## Results

Not including the time it takes to compile the C and Rust code, the native
optimizer is consistently 1-2 orders of magnitude faster. Compilation time often
dominates (in which case it is not shown as an explicit bar in the chart), but
not always.

### MacBook Pro (13-inch, M1, 2020)

![3d-projection-fake-3d-linear-algebra](mac-arm/3d-projection-fake-3d-linear-algebra.svg)

![allShapes-dashedShapes](mac-arm/allShapes-dashedShapes.svg)

![circle-example-euclidean](mac-arm/circle-example-euclidean.svg)

![collinear-euclidean](mac-arm/collinear-euclidean.svg)

![congruent-triangles-euclidean](mac-arm/congruent-triangles-euclidean.svg)

![continuousmap-continuousmap](mac-arm/continuousmap-continuousmap.svg)

![hypergraph-hypergraph](mac-arm/hypergraph-hypergraph.svg)

![incenter-triangle-euclidean](mac-arm/incenter-triangle-euclidean.svg)

![lagrange-bases-lagrange-bases](mac-arm/lagrange-bases-lagrange-bases.svg)

![midsegment-triangles-euclidean](mac-arm/midsegment-triangles-euclidean.svg)

![non-convex-non-convex](mac-arm/non-convex-non-convex.svg)

![one-water-molecule-atoms-and-bonds](mac-arm/one-water-molecule-atoms-and-bonds.svg)

![parallel-lines-euclidean](mac-arm/parallel-lines-euclidean.svg)

![persistent-homology-persistent-homology](mac-arm/persistent-homology-persistent-homology.svg)

![points-around-line-shape-distance](mac-arm/points-around-line-shape-distance.svg)

![points-around-polyline-shape-distance](mac-arm/points-around-polyline-shape-distance.svg)

![points-around-star-shape-distance](mac-arm/points-around-star-shape-distance.svg)

![siggraph-teaser-euclidean-teaser](mac-arm/siggraph-teaser-euclidean-teaser.svg)

![small-graph-disjoint-rect-line-horiz](mac-arm/small-graph-disjoint-rect-line-horiz.svg)

![small-graph-disjoint-rects-large-canvas](mac-arm/small-graph-disjoint-rects-large-canvas.svg)

![small-graph-disjoint-rects-small-canvas](mac-arm/small-graph-disjoint-rects-small-canvas.svg)

![small-graph-disjoint-rects](mac-arm/small-graph-disjoint-rects.svg)

![tree-venn-3d](mac-arm/tree-venn-3d.svg)

![tree-venn](mac-arm/tree-venn.svg)

![two-vectors-perp-vectors-dashed](mac-arm/two-vectors-perp-vectors-dashed.svg)

![wet-floor-atoms-and-bonds](mac-arm/wet-floor-atoms-and-bonds.svg)

![wos-laplace-estimator-walk-on-spheres](mac-arm/wos-laplace-estimator-walk-on-spheres.svg)

![wos-nested-estimator-walk-on-spheres](mac-arm/wos-nested-estimator-walk-on-spheres.svg)

![wos-offcenter-estimator-walk-on-spheres](mac-arm/wos-offcenter-estimator-walk-on-spheres.svg)

![wos-poisson-estimator-walk-on-spheres](mac-arm/wos-poisson-estimator-walk-on-spheres.svg)

## Prerequisites

- [Node](https://nodejs.org/en/download/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Yarn](https://classic.yarnpkg.com/lang/en/docs/install/)

## Usage

All commands in this README assume your working directory is the directory
containing this README. You'll first need to clone the Penrose repo submodule:

```sh
git submodule update penrose
```

This experiment requires some changes to the Penrose repo, stored in
`penrose.patch`. To apply those changes, run this command:

```sh
git -C penrose apply ../penrose.patch
```

Then to build and run the experiment, run this command:

```sh
./run.sh
```

Results should appear in the `results/` folder.

## Development

If you make further changes and want to re-export them to the patch file, run
this command:

```sh
git -C penrose diff > penrose.patch
```

To clear changes to the Penrose repo (for instance, if you want to freshly
re-apply `penrose.patch`):

```sh
git -C penrose restore .
```

After running the experiment, if you want to verify that the Rust version of the
optimizer actually worked correctly, run this to regenerate `penrose/diagrams/`:

```sh
./render.sh
```

Once you're satisfied looking at the diagrams, you'll probably want to restore
them in case you're planning to use the above command to update `penrose.patch`:

```sh
git -C penrose restore diagrams
```
