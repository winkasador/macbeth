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

#[inline]
pub fn piece_from_id(id: i32) -> Piece {
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

#[inline]
pub fn color_from_id(id: i32) -> Color {
    if id >= 6 {
        return Color::Black;
    }

    return Color::White;
}