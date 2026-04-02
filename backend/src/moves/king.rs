use crate::board::{Board, Color};

const KING_OFFSETS: [(i8, i8); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

pub fn king_moves(board: &Board, from: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    KING_OFFSETS.iter()
        .map(|(dr, dc)| (from.0 + dr, from.1 + dc))
        .filter(|&(r, c)| (0..8).contains(&r) && (0..8).contains(&c))
        .filter(|&(r, c)| match board[r as usize][c as usize] {
            Some((_, piece_color)) => piece_color != color,
            None => true,
        })
        .collect()
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
    fn king_in_center_has_eight_moves() {
        let board = empty_board();
        let moves = king_moves(&board, (4, 4), White);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn king_in_corner_has_three_moves() {
        let board = empty_board();
        let moves = king_moves(&board, (0, 0), White);
        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&(0, 1)));
        assert!(moves.contains(&(1, 0)));
        assert!(moves.contains(&(1, 1)));
    }

    #[test]
    fn king_cannot_capture_own_piece() {
        let mut board = empty_board();
        board[3][3] = Some((Pawn, White));
        board[3][4] = Some((Pawn, White));
        let moves = king_moves(&board, (4, 4), White);
        assert!(!moves.contains(&(3, 3)));
        assert!(!moves.contains(&(3, 4)));
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn king_can_capture_enemy_piece() {
        let mut board = empty_board();
        board[3][3] = Some((Pawn, Black));
        let moves = king_moves(&board, (4, 4), White);
        assert!(moves.contains(&(3, 3)));
        assert_eq!(moves.len(), 8);
    }
}
