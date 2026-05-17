# niche

## latest
 - introduce a random step generator and expose it in the repl
 - move repl into a test to hide it from the main codebase
 - instantiate a struct and call methods on it

## next
 - expose a seeded random number generator in the repl
 - build up a random walk in the repl; introduce dir. `scm` at project level

 - build a .so 

## longer term
 - hide the repl behind a test so that it does not figure in the main codebase (i.e. steel-core is a dev dependency only
 - be able to test structs etc from several modules/libraries

See `plan.md`.

## usage

After

```sh
cargo build
```

try

```sh
./target/debug/niche 
```

and then

```scm
(define (sq x) (* x x))
(sq 2)
```
=>
    4

Or,

```sh
./target/release/niche square.scm
```

```scm
(map square '(1 2 4))
```
=>
    '(1 4 16)

## multi-line input

```scm
(define (cube x)
    (* x x x))

(cube 3)
```
=>
    27

## lambda

```scm
(foldl add 0.0 '(1.0 3.5))
(map (λ (x) (add 1.0 x)) '(2.5 5.0))
```

## instantiate a struct in the repl

```sh
cargo test repl -- --ignored --nocapture
```

Then,

```scm
(define w (make-walk))
(step! w '(1 0))
(walk-last w)
(walk-length w)
```

## entering λ etc in neovim

In insert mode, try `ctrl-k l*`. And `:he ctrl-k`.


### end
