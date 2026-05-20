# niche

A rusty random walk with an embedded scheme repl and a dynamic library for R.

## latest

 - move the repl to its own module, put it behind a feature flag
 - build a .so and link to it from R
 - build up a random walk in the repl; introduce dir. `scm` at project level
 - source (slurp?) a file from the repl
 - expose a seeded random number generator in the repl

## usage

### run `main`

```sh
cargo run -- --n=15 --seed=100335
```

### use the scheme repl

```sh
cargo run --features repl -- --repl
```

Then,

```scm
(define w (make-walk))
(define omega (make-rng 10031))
(for-each (lambda (_) (step! w (random-step omega))) (range 0 10))
(walk-last w)
(walk-length w)
(walk-path w)
```

Or,

```scm
,load scheme/walk.scm
```

### build the .so for R

```sh
cargo build --lib
```

### R

```R
source("R/niche.R")
rng  <- make_rng(42)
walk <- make_walk()

for (i in 1:100) step(walk, rng)

last(walk)
length(walk)
path(walk)
```

And maybe,

```R
plot(do.call(rbind, path(walk)), type = 'l', asp = 1, xlab = '', ylab = '')
```

## entering λ etc in neovim

In insert mode, try `ctrl-k l*`. And `:he ctrl-k`.


### end
