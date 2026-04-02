use crate::board::{Board, Color};

const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (-2, -1), (-2, 1),
    (-1, -2), (-1, 2),
    ( 1, -2), ( 1, 2),
    ( 2, -1), ( 2, 1),
];

pub fn knight_moves(board: &Board, from: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    KNIGHT_OFFSETS.iter()
        .map(|(dr, dc)| (from.0 + dr, from.1 + dc))
        .filter(|&(r, c)| r >= 0 && r < 8 && c >= 0 && c < 8)
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
    fn knight_in_center_has_eight_moves() {
        let board = empty_board();
        let moves = knight_moves(&board, (4, 4), White);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn knight_in_corner_has_two_moves() {
        let board = empty_board();
        let moves = knight_moves(&board, (0, 0), White);
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&(1, 2)));
        assert!(moves.contains(&(2, 1)));
    }

    #[test]
    fn knight_cannot_capture_own_piece() {
        let mut board = empty_board();
        board[2][3] = Some((Pawn, White));
        board[3][2] = Some((Pawn, White));
        let moves = knight_moves(&board, (4, 4), White);
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn knight_can_capture_enemy_piece() {
        let mut board = empty_board();
        board[2][3] = Some((Pawn, Black));
        let moves = knight_moves(&board, (4, 4), White);
        assert_eq!(moves.len(), 8);
        assert!(moves.contains(&(2, 3)));
    }
}
