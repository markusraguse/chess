#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub enum Piece { King, Queen, Rook, Bishop, Knight, Pawn }

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub enum Color { White, Black }

pub type Board = [[Option<(Piece, Color)>; 8]; 8];