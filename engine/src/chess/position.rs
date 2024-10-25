use super::piece;

pub struct Position {
    pub bitboards: [i64; 12],
    pub occupation_bitboard: i64,
    pub castling_rights: [bool; 4],
    pub en_passant_index: i32,
    pub side_to_move: piece::Color
}

impl Position {
    pub fn new() -> Self {
        Position {
            bitboards: [0; 12],
            occupation_bitboard: 0,
            castling_rights: [false; 4],
            en_passant_index: -1,
            side_to_move: piece::Color::White
        }
    }
}

pub enum CastlingDirection {
    Short = 0,
    Long = 1,
}

pub fn is_square_occupied(position: &Position, index: i32) -> bool {
    position.occupation_bitboard & (1 << index) != 0
}

pub fn is_square_occupied_by_color(position: &Position, index: i32, color: piece::Color) -> bool {
    get_piece_at(position, index).1 == color
}

pub fn get_piece_at(position: &Position, index: i32) -> (piece::Piece, piece::Color) {
    if !is_square_occupied(position, index) { // Faster overall
        return (piece::Piece::Empty, piece::Color::None);
    }

    for bitboard_index in 0..position.bitboards.len() {
        if position.bitboards[bitboard_index] & (1 << index) != 0 {
            let piece_id = bitboard_index as i32;
            return (piece::piece_from_id(piece_id), piece::color_from_id(piece_id));
        }
    }

    return (piece::Piece::Empty, piece::Color::None); // shouldn't happen
}

pub fn set_piece_at(position: &mut Position, index: i32, piece: piece::Piece, color: piece::Color) {
    if piece == piece::Piece::Empty || color == piece::Color::None {
        return;
    }

    let mut target_bitboard = piece as i32;
    if color == piece::Color::Black {
        target_bitboard += 6;
    }

    position.occupation_bitboard = position.occupation_bitboard | (1 << index);
    position.bitboards[target_bitboard as usize] = position.bitboards[target_bitboard as usize] | (1 << index);
}