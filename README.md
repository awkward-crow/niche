# niche

[![CI](https://github.com/awkward-crow/niche/actions/workflows/ci.yml/badge.svg)](https://github.com/awkward-crow/niche/actions/workflows/ci.yml)

A 2D random walk in Rust, surfaced through three interfaces: a CLI binary, an interactive Steel/Scheme REPL and a native shared library callable from R.

The walk logic lives in `src/walk.rs`. The rest of the codebase is about the boundaries — compiling the same core as a `cdylib` for R via [extendr](https://extendr.github.io/) and registering Rust types as first-class objects in a [Steel](https://github.com/mattwparas/steel) Scheme VM behind a Cargo feature flag.

## usage

### CLI

```sh
cargo run -- --n=15 --seed=100335
```

### Scheme REPL

```sh
cargo run --features repl -- --repl
```

```scm
(define w (make-walk))
(define omega (make-rng 10031))
(for-each (lambda (_) (step! w (random-step omega))) (range 0 10))
(walk-last w)
(walk-length w)
(walk-path w)
```

Or load a script:

```scm
,load scheme/walk.scm
```

### R

Build the shared library:

```sh
cargo build --lib --release
```

```R
source("R/niche.R")
rng  <- make_rng(42)
walk <- make_walk()

for (i in 1:100) step(walk, rng)

last(walk)
length(walk)
path(walk)
```

```R
plot(do.call(rbind, path(walk)), type = 'l', asp = 1, xlab = '', ylab = '')
```
