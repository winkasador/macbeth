mod chess;

use chess::Position;

fn main() {
    let mut position = Position::new();

    println!("A: {:#?}", chess::get_piece_at(&position, 13));
    chess::set_piece_at(&mut position, 13, chess::Piece::Knight, chess::Color::Black);
    println!("B: {:#?}", chess::get_piece_at(&position, 13));
}