# scheme

Example scripts for the embedded Steel/Scheme REPL.

## running the repl

```sh
cargo run --features repl -- --repl
```

## walk session

```scm
(define w (make-walk))
(define omega (make-rng 10031))
(for-each (lambda (_) (step! w (random-step omega))) (range 0 10))
(walk-last w)
(walk-length w)
(walk-path w)
```

Or load the script directly:

```scm
,load scheme/walk.scm
```

## multi-line input

```scm
(define (cube x)
    (* x x x))

(cube 3)
```
=> 27

## lambda

```scm
(foldl add 0.0 '(1.0 3.5))
(map (λ (x) (add 1.0 x)) '(2.5 5.0))
```
