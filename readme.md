# niche

## latest
 - multi-line input in the repl
 - register a rust function and call it from the repl
 - read source files from the command line
 - repl 

## next

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

### entering λ etc in neovim

In insert mode, try `ctrl-k l*`. And `:he ctrl-k`.


### end
