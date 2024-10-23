#[derive(PartialEq, Debug)]
pub enum Piece {
    Empty = -1,
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[derive(PartialEq, Debug)]
pub enum Color {
    None = -1,
    White = 0,
    Black = 1,
}

pub enum CastlingDirection {
    Short = 0,
    Long = 1,
}

pub enum PromotionOption {
    None,
    Knight,
    Bishop,
    Rook,
    Queen
}

pub struct Move {
    from: i32,
    to: i32,
    promotion: PromotionOption
}

pub struct Position {
    bitboards: [i32; 12],
    occupation_bitboard: i32,
    castling_rights: [bool; 4],
    en_passant_index: i32,
}

impl Position {
    pub fn new() -> Self {
        Position {
            bitboards: [0; 12],
            occupation_bitboard: 0,
            castling_rights: [false; 4],
            en_passant_index: -1
        }
    }
}

pub fn set_piece_at(position: &mut Position, index: i32, piece: Piece, color: Color) {
    if piece == Piece::Empty || color == Color::None {
        return;
    }

    let mut target_bitboard = piece as i32;
    if color == Color::Black {
        target_bitboard += 6;
    }

    position.bitboards[target_bitboard as usize] = position.bitboards[target_bitboard as usize] | (1 << index);
}

pub fn is_square_occupied(position: &Position, index: i32) -> bool {
    for bitboard in position.bitboards {
        if (bitboard & (1 << index)) != 0 {
            return true;
        }
    }

    false
}

pub fn get_piece_at(position: &Position, index: i32) -> (Piece, Color) {
    if !is_square_occupied(position, index) { // Faster overall
        return (Piece::Empty, Color::None);
    }

    for bitboard_index in 0..position.bitboards.len() {
        if position.bitboards[bitboard_index] & (1 << index) != 0 {
            let piece_id = bitboard_index as i32;
            return (piece_from_id(piece_id), color_from_id(piece_id));
        }
    }

    return (Piece::Empty, Color::None); // shouldn't happen
}

// Utility Functions!

fn piece_from_id(id: i32) -> Piece {
    let mut id_ca = id; // adjust id to not depend on color. white pawn is 0 and a black one is 6, so subtract 6 to equalize.
    if id >= 6  {
        id_ca -= 6;
    }

    match id_ca {
        -1 => Piece::Empty,

        0 => Piece::Pawn,
        1 => Piece::Knight,
        2 => Piece::Bishop,
        3 => Piece::Rook,
        4 => Piece::Queen,
        5 => Piece::King,

        _ => Piece::Empty,
    }
}

fn color_from_id(id: i32) -> Color {
    if id >= 6 {
        return Color::Black;
    }

    return Color::White;
}