use piece::Piece;

use super::*;

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

pub fn piece_from_notation(piece: char) -> (piece::Piece, piece::Color) {
    let color = if piece.is_uppercase() { piece::Color::White } else { piece::Color::Black };
    let piece = match char::to_ascii_lowercase(&piece) {
        'p' => { piece::Piece::Pawn },
        'n' => { piece::Piece::Knight },
        'b' => { piece::Piece::Bishop },
        'r' => { piece::Piece::Rook },
        'q' => { piece::Piece::Queen },
        'k' => { piece::Piece::King },
        _ => { piece::Piece::Empty }
    };

    (piece, color)
}

pub fn board_index_to_coordinate_name(index: i32) -> String {
    let rank = index / 8;
    let file = index % 8 + 97;

    format!("{}{}", char::from(file as u8), rank + 1)
}

pub fn coordinate_name_to_board_index(coordinate: String) -> Option<i32> {
    if coordinate.len() != 2 {
        return None;
    }

    let file = coordinate.chars().nth(0)?;
    let file_index = match file {
        'a'..='h' => file as i32 - 'a' as i32,
        _ => return None,
    };

    // Convert rank (number) part to an index (0 to 7)
    let rank = coordinate.chars().nth(1)?;
    let rank_index = match rank {
        '1'..='8' => 7 - (rank as i32 - '1' as i32),
        _ => return None,
    };

    Some(rank_index * 8 + file_index)
}

pub fn list_moves(moves: &Vec<moves::Move>) -> String {
    let formatted_moves: Vec<String> = moves.iter()
        .map(|mv| mv.to_string())
        .collect();

    format!("{}", formatted_moves.join(", "))
}