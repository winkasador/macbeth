use position::Position;

use super::*;

pub fn position_from_fen(fen: &str) -> position::Position {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    assert_eq!(parts.len(), 6, "FEN must have 6 parts.");

    let mut position = position::Position::new();

    let ranks: Vec<&str> = parts[0].split("/").collect();
    for rank_index in 0..8 {
        let mut file = 0;
        for char in ranks[rank_index].chars() {
            if char::is_numeric(char) {
                file += char.to_digit(10).unwrap();
            }
            else {
                let piece = display::piece_from_notation(char);
                position::set_piece_at(&mut position, (7 - rank_index as i32) * 8 + (file as i32), piece.0, &piece.1);
                file += 1;
            }
        }   
    }

    position.side_to_move = match parts[1] {
        "w" => piece::Color::White,
        "b" => piece::Color::Black,
        _ => panic!("Invalid color in FEN!")
    };

    if parts[2] != "-" {
        if parts[2].contains("K") { position::set_castling_rights(&mut position, piece::Color::White, position::CastlingSide::Short, true); }
        if parts[2].contains("Q") { position::set_castling_rights(&mut position, piece::Color::White, position::CastlingSide::Long, true); }
        if parts[2].contains("k") { position::set_castling_rights(&mut position, piece::Color::Black, position::CastlingSide::Short, true); }
        if parts[2].contains("q") { position::set_castling_rights(&mut position, piece::Color::Black, position::CastlingSide::Long, true); }
    }

    let en_passant_option = display::coordinate_name_to_board_index(parts[3].to_string());
    position.en_passant_index = en_passant_option.unwrap_or(-1);

    position.half_move_clock = parts[4].parse::<i32>().unwrap();
    position.full_move_clock = parts[5].parse::<i32>().unwrap();

    position
}