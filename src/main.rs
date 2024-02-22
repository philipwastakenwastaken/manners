mod piece;

use crate::piece::{Piece, Color, PieceType};


fn main() {
    let p = Piece::make(PieceType::King, Color::Black);
    
    println!("Type: {:?}, Color: {:?}", p.piece_type(), p.color());
}
