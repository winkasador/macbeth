use std::fmt;

use super::display;
use super::piece;
use super::position;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PromotionOption {
    None,
    Knight,
    Bishop,
    Rook,
    Queen
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    from: i32,
    to: i32,
    promotion: PromotionOption
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Convert the `from` and `to` indices to algebraic notation
        let from_coordinate = display::board_index_to_coordinate_name(self.from);
        let to_coordinate = display::board_index_to_coordinate_name(self.to);

        // Format the promotion if applicable
        match self.promotion {
            PromotionOption::None => write!(f, "{}{}", from_coordinate, to_coordinate),
            PromotionOption::Knight => write!(f, "{}{}n", from_coordinate, to_coordinate),
            PromotionOption::Bishop => write!(f, "{}{}b", from_coordinate, to_coordinate),
            PromotionOption::Rook => write!(f, "{}{}r", from_coordinate, to_coordinate),
            PromotionOption::Queen => write!(f, "{}{}q", from_coordinate, to_coordinate),
        }
    }
}

/// Gets all moves a pawn could theoretically make, regardless of if they are legal or not.
pub fn get_pawn_unchecked_moves(position: &position::Position, index: i32, color: &piece::Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let rank = index / 8;
    let pawn_home_rank = if *color == piece::Color::White { 1 } else { 6 }; 
    let direction = if *color == piece::Color::White { 1 } else { -1 };
    let opponent_color = if *color == piece::Color::White { piece::Color::Black } else { piece::Color::White };

    if !position::is_square_occupied(position, index + 8 * direction) {
        add_pawn_move(&mut moves, index, index + 8 * direction, &color);

        if rank == pawn_home_rank && !position::is_square_occupied(position, index + 16 * direction) {
            moves.push(Move {from: index, to: index + 16 * direction, promotion: PromotionOption::None});
        }
    }
    
    if position::is_square_occupied_by_color(position, index + 7 * direction, &opponent_color) {
        let opponent_rank = (index + 7 * direction) / 8;
        if opponent_rank == rank + 1 * direction {
            add_pawn_move(&mut moves, index, index + 7 * direction, &color);
        }
    }
    if index + 9 * direction < 64 && index + 9 * direction >= 0 && position::is_square_occupied_by_color(position, index + 9 * direction, &opponent_color) {
        let opponent_rank = (index + 9 * direction) / 8;
        if opponent_rank == rank + 1 * direction {
            add_pawn_move(&mut moves, index, index + 9 * direction, &color);
        }
    }

    moves
}

fn add_pawn_move(moves: &mut Vec<Move>, from: i32, to: i32, color: &piece::Color) {
    let promotion_rank = if *color == piece::Color::White { 7 } else { 0 };
    let direction = if *color == piece::Color::White { 1 } else { -1 };
    let rank = (to * direction) / 8;

    if rank == promotion_rank {
        moves.append(&mut create_promotion_options(from, to));
    }
    else {
        moves.push(Move {from: from, to: to, promotion: PromotionOption::None});
    }
}

fn create_promotion_options(from: i32, to: i32) -> Vec<Move> {
    let mut promotion_options: Vec<Move> = Vec::new();

    promotion_options.push(Move {from: from, to: to, promotion: PromotionOption::Knight});
    promotion_options.push(Move {from: from, to: to, promotion: PromotionOption::Bishop});
    promotion_options.push(Move {from: from, to: to, promotion: PromotionOption::Rook});
    promotion_options.push(Move {from: from, to: to, promotion: PromotionOption::Queen});

    promotion_options
}

// Unit Tests! //

mod tests {
    use super::*;

    #[test]
    fn test_pawn_move_generation() {
        // This test is done with pawns on every square, including ones that aren't allowed (besides starting on promotion ranks).
        // This is so weird starting positions won't crash the program, but there's enough flexibility to actually support them.

        for color_index in 0..2 {
            let color = piece::color_from_id(color_index);
            let opponent_color = if color == piece::Color::White { piece::Color::Black } else { piece::Color::White };
            let direction = if color == piece::Color::White { 1 } else { -1 };
            let pawn_home_rank = if color == piece::Color::White { 1 } else { 6 };
            let promotion_rank = if color == piece::Color::White { 7 } else { 0 };
            let mut position = position::Position::new();

            for board_index in 0..64 {
                let rank = board_index / 8;
                let file = board_index % 8;
                if rank == promotion_rank { continue; }

                let mut generated_moves: Vec<Move>;
                let mut expected_moves: Vec<Move> = Vec::new();

                for test_index in 0..3 {
                    position::set_piece_at(&mut position, board_index, piece::Piece::Pawn, &color);

                    match test_index {
                        0 => { // Moving forward, no obstacles.
                            if rank == pawn_home_rank {
                                expected_moves.push(Move {from: board_index, to: board_index + 8 * direction, promotion: PromotionOption::None});
                                expected_moves.push(Move {from: board_index, to: board_index + 16 * direction, promotion: PromotionOption::None});
                            }
                            else {
                                add_pawn_move(&mut expected_moves, board_index, board_index + 8 * direction, &color);
                            }
                        }
                        1 => { // Obstacle
                            position::set_piece_at(&mut position, board_index + 8 * direction, piece::Piece::Rook, &opponent_color);
                        }
                        2 => { // Capture Left and Right
                            if rank == pawn_home_rank {
                                expected_moves.push(Move {from: board_index, to: board_index + 8 * direction, promotion: PromotionOption::None});
                                expected_moves.push(Move {from: board_index, to: board_index + 16 * direction, promotion: PromotionOption::None});
                            }
                            else {
                                add_pawn_move(&mut expected_moves, board_index, board_index + 8 * direction, &color);
                            }

                            let do_left = file != 0;
                            let do_right = file != 7;

                            if do_left { 
                                position::set_piece_at(&mut position, board_index + 7 * direction, piece::Piece::Pawn, &opponent_color); 
                                add_pawn_move(&mut expected_moves, board_index, board_index + 7 * direction, &color);
                            }
                            else if do_right {
                                position::set_piece_at(&mut position, board_index + 9 * direction, piece::Piece::Pawn, &opponent_color);
                                add_pawn_move(&mut expected_moves, board_index, board_index + 9 * direction, &color);
                            }
                        }
                        _ => {
                            panic!("Test misconfigured. There is no scenario {}.", test_index);
                        }
                    }

                    generated_moves = get_pawn_unchecked_moves(&position, board_index, &color);

                    generated_moves.sort();
                    expected_moves.sort();

                    assert!(generated_moves == expected_moves, "Generated moves for the following position do not match expectation.\n{}\nExpected: {}\nGot: {}",
                    display::ascii(&position),
                    display::list_moves(&expected_moves),
                    display::list_moves(&generated_moves));

                    expected_moves.clear();
                    generated_moves.clear();

                    position::massacre(&mut position);
                }
            }
        }
    }
}