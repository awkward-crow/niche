# niche

## latest

 - build a .so and link to it from R
 - build up a random walk in the repl; introduce dir. `scm` at project level
 - source (slurp?) a file from the repl
 - expose a seeded random number generator in the repl
 - introduce a random step generator and expose it in the repl
 - move repl into a test to hide it from the main codebase
 - instantiate a struct and call methods on it

## usage

### run `main`

```sh
cargo run -- --n=15 --seed=100335
```

### use the repl

```sh
cargo test repl -- --ignored --nocapture
```

Then,

```scm
(define w (make-walk))
(define omega (make-rng 10031))
(for-each (lambda (_) (step! w (random-step omega))) (range 0 10))
(walk-last w)
(walk-length w)    # list of 101 c(x, y) pairs
(walk-path w)
```

Or,

```scm
,load scheme/walk.scm
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
