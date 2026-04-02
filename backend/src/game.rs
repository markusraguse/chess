use serde::Serialize;
use crate::board::{Board, Color, Piece};

#[derive(Clone, Serialize)]
pub struct GameState {
    pub board: Board,
    pub turn: Color,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: initial_board(),
            turn: Color::White,
        }
    }
}

fn initial_board() -> Board {
    use Piece::*;
    use Color::*;
    [
        [Some((Rook,Black)),Some((Knight,Black)),Some((Bishop,Black)),Some((Queen,Black)),Some((King,Black)),Some((Bishop,Black)),Some((Knight,Black)),Some((Rook,Black))],
        [Some((Pawn,Black));8],
        [None;8],
        [None;8],
        [None;8],
        [None;8],
        [Some((Pawn,White));8],
        [Some((Rook,White)),Some((Knight,White)),Some((Bishop,White)),Some((Queen,White)),Some((King,White)),Some((Bishop,White)),Some((Knight,White)),Some((Rook,White))],
    ]
}
