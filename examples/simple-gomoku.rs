use std::io::Write;

use itertools::Itertools;

use gomoku_core::*;

fn visualize<const X: usize, const Y: usize>(board: &[[Option<Player>; X]; Y]) {
    let board_str = board
        .iter()
        .map(|v| {
            v.iter()
                .map(|p| match p {
                    Some(Player::Black) => "[#]",
                    Some(Player::White) => "[ ]",
                    None => " . ",
                })
                .join("")
        })
        .enumerate()
        .map(|(n, v)| format!("{n:02} {v}"))
        .join("\n");

    println!("   {}", (0..X).map(|v| format!("{v:02}")).join(" "));
    println!("{board_str}");
}

fn main() {
    let mut session: Session<13, 13, 5> = Session::default();

    loop {
        println!();
        visualize(session.cells());
        println!("Turn: {:?}", session.player());

        let ask = |msg| loop {
            let mut s = String::new();
            print!("{msg}? ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut s).unwrap();
            match s.trim().parse() {
                Ok(v) => return v,
                Err(_) => {}
            }
        };

        let state = loop {
            let c = loop {
                match Coordinate::try_new(ask("X"), ask("Y")) {
                    Ok(v) => break v,
                    _ => println!("Coordinate is out of board, select again."),
                }
            };

            match session.put(c) {
                Ok(v) => break v,
                _ => println!("Cannot place here. This place has a stone already."),
            }
        };

        match state {
            PlayState::Continue(s) => {
                session = s;
            }
            PlayState::HasWinner((player, cells)) => {
                visualize(&cells);
                println!("Winner: {player:?}");
                break;
            }
            PlayState::RemainCellsIsZero(cells) => {
                visualize(&cells);
                println!("Draw!");
                break;
            }
        }
    }
}
