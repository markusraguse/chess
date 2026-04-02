use crate::board::{Board, Color};

const ROOK_DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn rook_moves(board: &Board, from: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();

    for (dr, dc) in ROOK_DIRECTIONS {
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
    fn rook_in_corner_on_empty_board() {
        let board = empty_board();
        let moves = rook_moves(&board, (0, 0), White);
        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn rook_in_center_on_empty_board() {
        let board = empty_board();
        let moves = rook_moves(&board, (4, 4), White);
        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn rook_blocked_by_own_piece() {
        let mut board = empty_board();
        board[4][6] = Some((Pawn, White));
        let moves = rook_moves(&board, (4, 4), White);
        // Right ray stops before (4,6), so (4,5) only — loses 2 squares on that ray
        assert!(!moves.contains(&(4, 6)));
        assert!(!moves.contains(&(4, 7)));
        assert!(moves.contains(&(4, 5)));
    }

    #[test]
    fn rook_can_capture_enemy_piece() {
        let mut board = empty_board();
        board[4][6] = Some((Pawn, Black));
        let moves = rook_moves(&board, (4, 4), White);
        assert!(moves.contains(&(4, 6)));  // capture square included
        assert!(!moves.contains(&(4, 7))); // but not beyond
    }

    #[test]
    fn rook_cannot_pass_through_pieces() {
        let mut board = empty_board();
        board[4][5] = Some((Pawn, Black));
        board[4][6] = Some((Pawn, Black));
        let moves = rook_moves(&board, (4, 4), White);
        assert!(moves.contains(&(4, 5)));  // captures first blocker
        assert!(!moves.contains(&(4, 6))); // cannot reach second
    }
}
