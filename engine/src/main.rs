pub mod chess;

use chess::*;

fn main() {
    let mut position = fen::position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{}", display::ascii(&position));
}