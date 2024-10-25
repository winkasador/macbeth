use super::position;
use super::piece;

pub fn ascii(position: &position::Position) -> String {
    let mut ascii: String = "  +------------------------+\n".to_owned();

    for rank in (0..8).rev() {
        ascii.push_str(&(rank + 1).to_string());
        ascii.push_str(" |");

        for file in 0..8 {
            let piece_and_color: (piece::Piece, piece::Color) = position::get_piece_at(position, rank * 8 + file);
            

            ascii.push(' ');
            ascii.push(get_piece_ascii(piece_and_color.0, piece_and_color.1));
            ascii.push(' ');
        }

        ascii.push_str("|\n");
    }

    ascii.push_str("  +------------------------+\n");
    ascii.push_str("    a  b  c  d  e  f  g  h");

    ascii
}

pub fn get_piece_ascii(piece: piece::Piece, color: piece::Color) -> char {
    let mut piece_ascii = ' ';

    match piece {
        piece::Piece::Empty => piece_ascii = '.',

        piece::Piece::Pawn => piece_ascii = 'p',
        piece::Piece::Knight => piece_ascii = 'n',
        piece::Piece::Bishop => piece_ascii = 'b',
        piece::Piece::Rook => piece_ascii = 'r',
        piece::Piece::Queen => piece_ascii = 'q',
        piece::Piece::King => piece_ascii = 'k',

        _ => piece_ascii = '.'
    }

    if color == piece::Color::White {
        piece_ascii = piece_ascii.to_ascii_uppercase();
    }

    return piece_ascii;
}