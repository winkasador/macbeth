use super::position;
use super::piece;

pub fn position_from_fen(fen: &str) -> position::Position {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    assert_eq!(parts.len(), 6, "FEN must have 6 parts.");

    let mut position = position::Position::new();

    position.side_to_move = match parts[1] {
        "w" => piece::Color::White,
        "b" => piece::Color::Black,
        _ => panic!("Invalid color in FEN!")
    };

    position
}