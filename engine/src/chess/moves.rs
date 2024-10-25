use super::piece;
use super::position;

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

/// Gets all moves a pawn could theoretically make, regardless of if they are legal or not.
pub fn get_pawn_unchecked_moves(position: &position::Position, index: i32, color: piece::Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let rank = index / 8;

    if color == piece::Color::White {
        if !position::is_square_occupied(position, index + 8) {
            if rank == 6 {
                moves.append(&mut create_promotion_options(index, index + 8));
            }
            else {
                moves.push(Move {from: index, to: index + 8, promotion: PromotionOption::None});
    
                if rank == 1 && !position::is_square_occupied(position, index + 16) {
                    moves.push(Move {from: index, to: index + 16, promotion: PromotionOption::None});
                }
            }
        }
    
        if position::is_square_occupied_by_color(position, index + 7, piece::Color::Black) {
            let opponent_rank = (index + 7) / 8;
            if opponent_rank == rank + 1 {
                if opponent_rank == 7 {
                    moves.append(&mut create_promotion_options(index, index + 7));
                }
                else {
                    moves.push(Move {from: index, to: index + 7, promotion: PromotionOption::None});
                }
            }
        }
        if index + 9 < 64 && position::is_square_occupied_by_color(position, index + 9, piece::Color::Black) {
            let opponent_rank = (index + 9) / 8;
            if opponent_rank == rank + 1 {
                if opponent_rank == 7 {
                    moves.append(&mut create_promotion_options(index, index + 9));
                }
                else {
                    moves.push(Move {from: index, to: index + 9, promotion: PromotionOption::None});
                }
            }
        }
    }
    else if color == piece::Color::Black {
        if !position::is_square_occupied(position, index - 8) {
            if rank == 1 {
                moves.append(&mut create_promotion_options(index, index - 8));
            }
            else {
                moves.push(Move {from: index, to: index - 8, promotion: PromotionOption::None});
    
                if rank == 6 && !position::is_square_occupied(position, index - 16) {
                    moves.push(Move {from: index, to: index - 16, promotion: PromotionOption::None});
                }
            }
        }
    
        if position::is_square_occupied_by_color(position, index - 7, piece::Color::White) {
            let opponent_rank = (index - 7) / 8;
            if opponent_rank == rank - 1 {
                if opponent_rank == 0 {
                    moves.append(&mut create_promotion_options(index, index - 7));
                }
                else {
                    moves.push(Move {from: index, to: index + 7, promotion: PromotionOption::None});
                }
            }
        }
        if index - 9 >= 0 && position::is_square_occupied_by_color(position, index - 9, piece::Color::White) {
            let opponent_rank = (index - 9) / 8;
            if opponent_rank == rank - 1 {
                if opponent_rank == 0 {
                    moves.append(&mut create_promotion_options(index, index - 9));
                }
                else {
                    moves.push(Move {from: index, to: index - 9, promotion: PromotionOption::None});
                }
            }
        }
    }

    moves
}

fn create_promotion_options(from: i32, to: i32) -> Vec<Move> {
    let mut promotion_options: Vec<Move> = Vec::new();

    promotion_options.push(Move {from: from, to: to, promotion: PromotionOption::Knight});
    promotion_options.push(Move {from: from, to: to, promotion: PromotionOption::Bishop});
    promotion_options.push(Move {from: from, to: to, promotion: PromotionOption::Rook});
    promotion_options.push(Move {from: from, to: to, promotion: PromotionOption::Queen});

    promotion_options
}