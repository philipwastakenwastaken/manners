use num_derive::FromPrimitive;
use proc_bitfield::bitfield;

#[derive(FromPrimitive, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        let color_opt = num::FromPrimitive::from_u8(value);
        assert!(color_opt.is_some());

        color_opt.unwrap()
    }
}

#[derive(FromPrimitive, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl From<u8> for PieceType {
    fn from(value: u8) -> Self {
        let piece_opt = num::FromPrimitive::from_u8(value);
        assert!(piece_opt.is_some());

        piece_opt.unwrap()
    }
}

bitfield! {

    #[derive(Debug, PartialEq)]
    pub struct Piece(pub u8) {
        pub piece_type: u8 [get PieceType] @ 0..=6,
        pub color: u8 [get Color] @ 7..=7
    }

}

impl Piece {
    pub fn make(piece_type: PieceType, color: Color) -> Self {
        let a = 0x0;
        let mut p = Piece(a);

        p.set_color(color as u8);
        p.set_piece_type(piece_type as u8);

        p
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Piece, PieceType};

    #[test]
    fn test_make() {
        let p1 = Piece::make(PieceType::Pawn, Color::White);
        let p2 = Piece::make(PieceType::Pawn, Color::Black);

        assert_eq!(p1.color(), Color::White);
        assert_eq!(p2.color(), Color::Black);

        assert_eq!(p1.piece_type(), PieceType::Pawn);
        assert_eq!(p2.piece_type(), PieceType::Pawn);

        assert_ne!(p1, p2);
    }
}
