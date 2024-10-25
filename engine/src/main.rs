pub mod chess;

use chess::fen;
use chess::piece;
use chess::display;
use chess::moves;
use chess::position;

fn main() {
    let mut position = fen::position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    let index = 47;

    position::set_piece_at(&mut position, index, piece::Piece::Pawn, piece::Color::Black);
    println!("{}", display::ascii(&position));
    println!("{} move(s).", moves::get_pawn_unchecked_moves(&position, index, piece::Color::Black).len());
}