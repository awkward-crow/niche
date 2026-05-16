# niche

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



### end
