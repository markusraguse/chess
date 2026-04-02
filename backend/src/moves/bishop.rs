use crate::board::{Board, Color};

const BISHOP_DIRECTIONS: [(i8, i8); 4] = [(-1, -1), (1, 1), (1, -1), (-1, 1)];

pub fn bishop_moves(board: &Board, from: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();

    for (dr, dc) in BISHOP_DIRECTIONS {
        let mut r = from.0 + dr;
        let mut c = from.1 + dc;

        while (0..8).contains(&r) && (0..8).contains(&c) {
            match board[r as usize][c as usize] {
                None => {
                    moves.push((r, c));
                }
                Some((_, piece_color)) => {
                    if piece_color != color {
                        moves.push((r, c)); // can capture
                    }
                    break; // blocked either way
                }
            }
            r += dr;
            c += dc;
        }
    }

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
    fn bishop_in_corner_on_empty_board() {
        let board = empty_board();
        let moves = bishop_moves(&board, (0, 0), White);
        assert_eq!(moves.len(), 7); // one diagonal from corner
    }

    #[test]
    fn bishop_in_center_on_empty_board() {
        let board = empty_board();
        let moves = bishop_moves(&board, (4, 4), White);
        assert_eq!(moves.len(), 13);
    }

    #[test]
    fn bishop_blocked_by_own_piece() {
        let mut board = empty_board();
        board[2][2] = Some((Pawn, White));
        let moves = bishop_moves(&board, (4, 4), White);
        assert!(!moves.contains(&(2, 2)));
        assert!(!moves.contains(&(1, 1)));
        assert!(moves.contains(&(3, 3))); // square before blocker still reachable
    }

    #[test]
    fn bishop_can_capture_enemy_piece() {
        let mut board = empty_board();
        board[2][2] = Some((Pawn, Black));
        let moves = bishop_moves(&board, (4, 4), White);
        assert!(moves.contains(&(2, 2)));  // capture square included
        assert!(!moves.contains(&(1, 1))); // but not beyond
    }

    #[test]
    fn bishop_cannot_pass_through_pieces() {
        let mut board = empty_board();
        board[3][3] = Some((Pawn, Black));
        board[2][2] = Some((Pawn, Black));
        let moves = bishop_moves(&board, (4, 4), White);
        assert!(moves.contains(&(3, 3)));  // captures first blocker
        assert!(!moves.contains(&(2, 2))); // cannot reach second
    }
}