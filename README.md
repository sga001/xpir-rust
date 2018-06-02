# Rust bindings for XPIR

We wrote this Rust wrapper as part of our [paper](https://eprint.iacr.org/2017/1142.pdf) in order to microbenchmark and use XPIR.

To install, you need all of XPIR's dependencies (boost >= 1.55, gmp, mpfr). You can install them as follows.

Ubuntu 14.04 (or later) / Debian

```sh
$ sudo apt-get install libboost1.55-all-dev libmpfr-dev libgmp-dev
```

Arch:

```sh
$ sudo pacman -S boost mpfr gmp
```

There is no need to compile XPIR. It will be built and linked automatically.

# Compiling XPIR rust bindings

```sh
$ git submodule init
$ git submodule update
$ cargo build
```

# Reproducing the results in the paper

If you would like to reproduce the microbenchmarks found in the paper (Figure 9), simply run:

```sh
$ cargo bench [prefix of name of benchmark (or leave blank to run all)]
```

For example, to reproduce the XPIR entries of the first row of Figure 9 (Query), simply
run: 

```sh
$ cargo bench query
```

To reproduce a single data point, for example the Answer entry for XPIR (d=2) where ``n=262,144``, run:

```sh
$ cargo bench reply_d2/262144
```

Note that to run the microbenchmakrs for ``d=3`` you need to manually modify the ``DIM`` variable in ``benches/pir.rs``. You can also find the code that runs these benchmarks (and their names) in that same file.

To reproduce latency and throughput results, check out the [pir-test](https://github.com/sga001/pir-test) repository (this also has examples on how to use XPIR-rust in a client-server networked application).
