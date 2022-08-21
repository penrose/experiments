# 2022 Optimizer Performance

author: @samestep

This experiment is an attempt to get an idea of how much faster the Penrose
optimizer could possibly be, without fundamentally changing how it works.

## Usage

All commands in this README assume your working directory is the directory
containing this README.

This experiment requires some changes to the Penrose repo, stored in
`penrose.patch`. To apply those changes, run this command:

```sh
git -C penrose apply ../penrose.patch
```

Then to build and run the experiment, run this command:

```sh
./run.sh
```

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
