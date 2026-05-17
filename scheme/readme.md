# scheme 

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

## see also

file `square.scm`


### end
