use super::piece::{self, Color};

pub struct Position {
    pub bitboards: [i64; 12],
    pub occupation_bitboard: i64,
    pub castling_rights: [bool; 4],
    pub en_passant_index: i32,
    pub side_to_move: piece::Color,
    pub half_move_clock: i32,
    pub full_move_clock: i32
}

impl Position {
    pub fn new() -> Self {
        Position {
            bitboards: [0; 12],
            occupation_bitboard: 0,
            castling_rights: [false; 4],
            en_passant_index: -1,
            side_to_move: piece::Color::White,
            half_move_clock: 0,
            full_move_clock: 1
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CastlingSide {
    Short = 0,
    Long = 1,
}

pub fn is_square_occupied(position: &Position, index: i32) -> bool {
    position.occupation_bitboard & (1 << index) != 0
}

pub fn is_square_occupied_by_color(position: &Position, index: i32, color: &piece::Color) -> bool {
    get_piece_at(position, index).1 == *color
}

pub fn set_castling_rights(position: &mut Position, color: Color, side: CastlingSide, is_available: bool) {
    let castling_index = (if side == CastlingSide::Short {0} else {1}) + (if color == Color::White { 0 } else { 2 } );
    position.castling_rights[castling_index] = is_available;
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

pub fn set_piece_at(position: &mut Position, index: i32, piece: piece::Piece, color: &piece::Color) {
    if piece == piece::Piece::Empty || *color == piece::Color::None {
        return;
    }

    let mut target_bitboard = piece as i32;
    if *color == piece::Color::Black {
        target_bitboard += 6;
    }

    position.occupation_bitboard = position.occupation_bitboard | (1 << index);
    position.bitboards[target_bitboard as usize] = position.bitboards[target_bitboard as usize] | (1 << index);
}

pub fn massacre(position: &mut Position) {
    position.occupation_bitboard = 0;
    position.bitboards = [0; 12];
}

// Unit Tests! //

mod tests {
    use super::*;

    #[test]
    fn test_piece_bitboards() {
        for color_index in 0..2 {
            for piece_index in 0..6 {
                for board_index in 0..64 {
                    let mut position = Position::new();
                    set_piece_at(&mut position, board_index, piece::piece_from_id(piece_index), &piece::color_from_id(color_index));

                    let piece = get_piece_at(&position, board_index);
                    assert!(piece.0 == piece::piece_from_id(piece_index) && piece.1 == piece::color_from_id(color_index), "Piece at board index {} was supposed to be a {:?} {:?} but was instead a {:?} {:?}.", board_index, piece::color_from_id(color_index), piece::piece_from_id(piece_index), piece.1, piece.0);
                }
            }
        }
    }

    #[test]
    fn test_occupation_bitboard() {
        for i in 0..64 {
            let mut position = Position::new();
            set_piece_at(&mut position, i, piece::Piece::Knight, &piece::Color::White);

            assert!(is_square_occupied(&position, i), "Board index {} should be considered occupied, but it isn't.", i);
            assert!(!is_square_occupied_by_color(&position, i, &piece::Color::Black), "Board index {} should be occupied by a white piece, but the check indicates it isn't.", i);
        }
    }
}