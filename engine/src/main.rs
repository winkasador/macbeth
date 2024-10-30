pub mod chess;

use chess::*;

fn main() {
    let mut position = fen::position_from_fen("8/8/8/8/4N3/8/8/8 w KQkq - 0 1");
    println!("{}", display::ascii(&position));
    println!("Legal Moves: {}", display::list_moves(&moves::get_unchecked_moves(&position)));
}