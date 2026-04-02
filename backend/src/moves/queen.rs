use crate::board::{Board, Color};
use super::{rook_moves, bishop_moves};

pub fn queen_moves(board: &Board, from: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    let mut moves = rook_moves(board, from, color);
    moves.extend(bishop_moves(board, from, color));
    moves
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Piece::*;
    use crate::board::Color::*;

    fn empty_board() -> Board {
        [[None; 8]; 8]
    }

    #[test]
    fn queen_in_center_on_empty_board() {
        let board = empty_board();
        let moves = queen_moves(&board, (4, 4), White);
        assert_eq!(moves.len(), 27); // 14 rook + 13 bishop
    }

    #[test]
    fn queen_in_corner_on_empty_board() {
        let board = empty_board();
        let moves = queen_moves(&board, (0, 0), White);
        assert_eq!(moves.len(), 21); // 14 rook + 7 bishop
    }

    #[test]
    fn queen_blocked_by_own_piece() {
        let mut board = empty_board();
        board[4][6] = Some((Pawn, White));
        let moves = queen_moves(&board, (4, 4), White);
        assert!(!moves.contains(&(4, 6)));
        assert!(!moves.contains(&(4, 7)));
    }

    #[test]
    fn queen_can_capture_enemy_piece() {
        let mut board = empty_board();
        board[4][6] = Some((Pawn, Black));
        let moves = queen_moves(&board, (4, 4), White);
        assert!(moves.contains(&(4, 6)));
        assert!(!moves.contains(&(4, 7)));
    }
}
