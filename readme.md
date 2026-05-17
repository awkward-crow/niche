# niche

## latest
 - introduce a random step generator and expose it in the repl
 - move repl into a test to hide it from the main codebase
 - instantiate a struct and call methods on it

## next
 - expose a seeded random number generator in the repl
 - build up a random walk in the repl; introduce dir. `scm` at project level
 - source (slurp?) a file from the repl

 - build a .so 

## longer term
 - hide the repl behind a test so that it does not figure in the main codebase (i.e. steel-core is a dev dependency only
 - be able to test structs etc from several modules/libraries

See `plan.md`.

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
(walk-length w)
(walk-path w)
```

## entering λ etc in neovim

In insert mode, try `ctrl-k l*`. And `:he ctrl-k`.


### end
