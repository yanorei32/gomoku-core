# Gomoku Core
A simple and typesafe Gomoku core library written in pure Rust.

This implementation doesn't have "Kinjite" currently.

## Core idea

```rs
let mut session: Session<13, 13, 5> = Session::default();
loop {
    let c = Coordinate::try_new(ask("X"), ask("Y")).unwrap();

    match session.put(c).unwrap() {
        PlayState::Continue(s) => {
            session = s;
        }
        PlayState::HasWinner((player, _cells)) => {
            println!("Winner: {player:?}");
            break;
        }
        PlayState::RemainCellsIsZero(_cells) => {
            println!("Draw!");
            break;
        }
    }
}
```

## Examples

### Gomoku

https://github.com/yanorei32/gomoku-core/blob/master/examples/simple-gomoku.rs

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

https://github.com/yanorei32/gomoku-core/blob/master/examples/simple-marubatsu.rs

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
