use crate::board::{Board, Color};

pub fn pawn_moves(board: &Board, from: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    let (r, c) = from;

    // White moves up (decreasing row), black moves down (increasing row)
    let direction: i8 = match color { Color::White => -1, Color::Black => 1 };
    let starting_rank: i8 = match color { Color::White => 6, Color::Black => 1 };

    // Single push — only if the square ahead is empty
    let one_ahead = r + direction;
    if (0..8).contains(&one_ahead) && board[one_ahead as usize][c as usize].is_none() {
        moves.push((one_ahead, c));

        // Double push — only from starting rank and if both squares ahead are empty
        let two_ahead = r + direction * 2;
        if r == starting_rank && board[two_ahead as usize][c as usize].is_none() {
            moves.push((two_ahead, c));
        }
    }

    // Diagonal captures — only if an enemy piece is there
    for dc in [-1i8, 1] {
        let target_r = r + direction;
        let target_c = c + dc;
        if !(0..8).contains(&target_r) || !(0..8).contains(&target_c) {
            continue;
        }
        if let Some((_, piece_color)) = board[target_r as usize][target_c as usize] {
            if piece_color != color {
                moves.push((target_r, target_c));
            }
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
    fn pawn_single_push_from_starting_rank() {
        let board = empty_board();
        let moves = pawn_moves(&board, (6, 4), White);
        assert!(moves.contains(&(5, 4)));
    }

    #[test]
    fn pawn_double_push_from_starting_rank() {
        let board = empty_board();
        let moves = pawn_moves(&board, (6, 4), White);
        assert!(moves.contains(&(4, 4)));
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn pawn_double_push_blocked_if_first_square_occupied() {
        let mut board = empty_board();
        board[5][4] = Some((Pawn, Black));
        let moves = pawn_moves(&board, (6, 4), White);
        assert!(moves.is_empty());
    }

    #[test]
    fn pawn_single_push_only_when_not_on_starting_rank() {
        let board = empty_board();
        let moves = pawn_moves(&board, (4, 4), White);
        assert_eq!(moves, vec![(3, 4)]);
    }

    #[test]
    fn pawn_cannot_push_if_blocked() {
        let mut board = empty_board();
        board[5][4] = Some((Pawn, White));
        let moves = pawn_moves(&board, (6, 4), White);
        assert!(moves.is_empty());
    }

    #[test]
    fn pawn_captures_diagonally() {
        let mut board = empty_board();
        board[5][3] = Some((Knight, Black));
        board[5][5] = Some((Knight, Black));
        let moves = pawn_moves(&board, (6, 4), White);
        assert!(moves.contains(&(5, 3)));
        assert!(moves.contains(&(5, 5)));
    }

    #[test]
    fn pawn_cannot_capture_own_piece() {
        let mut board = empty_board();
        board[5][3] = Some((Knight, White));
        let moves = pawn_moves(&board, (6, 4), White);
        assert!(!moves.contains(&(5, 3)));
    }

    #[test]
    fn black_pawn_moves_downward() {
        let board = empty_board();
        let moves = pawn_moves(&board, (1, 4), Black);
        assert!(moves.contains(&(2, 4)));
        assert!(moves.contains(&(3, 4)));
        assert_eq!(moves.len(), 2);
    }
}
