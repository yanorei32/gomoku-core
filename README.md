# Gomoku Core
A simple and typesafe Gomoku core library written in pure rust.
This implementation doesn't have "Kinjite" currently.

## Examples

### Gomoku

```
$ cargo run --example simple-gomoku
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/simple-gomoku`

   00 01 02 03 04 05 06 07 08 09 10 11 12
00  .  .  .  .  .  .  .  .  .  .  .  .  .
01  .  .  .  .  .  .  .  .  .  .  .  .  .
02  .  .  .  .  .  .  .  .  .  .  .  .  .
03  .  .  .  .  .  .  .  .  .  .  .  .  .
04  .  .  .  .  .  .  .  .  .  .  .  .  .
05  .  .  .  .  .  .  .  .  .  .  .  .  .
06  .  .  .  .  .  .  .  .  .  .  .  .  .
07  .  .  .  .  .  .  .  .  .  .  .  .  .
08  .  .  .  .  .  .  .  .  .  .  .  .  .
09  .  .  .  .  .  .  .  .  .  .  .  .  .
10  .  .  .  .  .  .  .  .  .  .  .  .  .
11  .  .  .  .  .  .  .  .  .  .  .  .  .
12  .  .  .  .  .  .  .  .  .  .  .  .  .
Turn: Black
X? 
```

### Marubatsu

```
$ cargo run --example simple-marubatsu
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/simple-marubatsu`

  0 1 2
0 . . .
1 . . .
2 . . .
Turn: O
X?
```
