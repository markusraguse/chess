mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub use bishop::bishop_moves;
pub use king::king_moves;
pub use knight::knight_moves;
pub use pawn::pawn_moves;
pub use queen::queen_moves;
pub use rook::rook_moves;

use crate::board::{Board, Color, Piece};

#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(tag = "type", content = "winner")]
pub enum GameStatus {
    Ongoing,
    Checkmate(Color),
    Stalemate,
}

pub fn game_status(board: &Board, turn: Color) -> GameStatus {
    if has_any_legal_moves(board, turn) {
        return GameStatus::Ongoing;
    }
    if is_in_check(board, turn) {
        let winner = match turn { Color::White => Color::Black, Color::Black => Color::White };
        GameStatus::Checkmate(winner)
    } else {
        GameStatus::Stalemate
    }
}

fn has_any_legal_moves(board: &Board, color: Color) -> bool {
    for r in 0..8i8 {
        for c in 0..8i8 {
            if let Some((_, piece_color)) = board[r as usize][c as usize] {
                if piece_color == color && !legal_moves(board, (r, c)).is_empty() {
                    return true;
                }
            }
        }
    }
    false
}

pub fn legal_moves(board: &Board, from: (i8, i8)) -> Vec<(i8, i8)> {
    let (r, c) = (from.0 as usize, from.1 as usize);
    let color = match board[r][c] {
        Some((_, color)) => color,
        None => return vec![],
    };

    let candidates = match board[r][c] {
        Some((Piece::Bishop, color)) => bishop_moves(board, from, color),
        Some((Piece::King,   color)) => king_moves(board, from, color),
        Some((Piece::Knight, color)) => knight_moves(board, from, color),
        Some((Piece::Pawn,   color)) => pawn_moves(board, from, color),
        Some((Piece::Queen,  color)) => queen_moves(board, from, color),
        Some((Piece::Rook,   color)) => rook_moves(board, from, color),
        None => vec![],
    };

    candidates.into_iter()
        .filter(|&to| !move_leaves_king_in_check(board, from, to, color))
        .collect()
}

fn move_leaves_king_in_check(board: &Board, from: (i8, i8), to: (i8, i8), color: Color) -> bool {
    let mut board = *board;
    board[to.0 as usize][to.1 as usize] = board[from.0 as usize][from.1 as usize];
    board[from.0 as usize][from.1 as usize] = None;
    is_in_check(&board, color)
}

pub fn is_in_check(board: &Board, color: Color) -> bool {
    let king_pos = find_king(board, color);
    is_square_attacked(board, king_pos, color)
}

fn find_king(board: &Board, color: Color) -> (i8, i8) {
    for r in 0..8 {
        for c in 0..8 {
            if board[r][c] == Some((Piece::King, color)) {
                return (r as i8, c as i8);
            }
        }
    }
    panic!("King not found on board");
}

/// Checks if a square is attacked by any enemy piece, by casting rays and
/// checking offsets outward from the square itself.
fn is_square_attacked(board: &Board, square: (i8, i8), color: Color) -> bool {
    let enemy = match color { Color::White => Color::Black, Color::Black => Color::White };

    // Check for attacking rooks or queens along ranks and files
    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let mut r = square.0 + dr;
        let mut c = square.1 + dc;
        while (0..8).contains(&r) && (0..8).contains(&c) {
            match board[r as usize][c as usize] {
                Some((Piece::Rook, col)) | Some((Piece::Queen, col)) if col == enemy => return true,
                Some(_) => break,
                None => {}
            }
            r += dr;
            c += dc;
        }
    }

    // Check for attacking bishops or queens along diagonals
    for (dr, dc) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
        let mut r = square.0 + dr;
        let mut c = square.1 + dc;
        while (0..8).contains(&r) && (0..8).contains(&c) {
            match board[r as usize][c as usize] {
                Some((Piece::Bishop, col)) | Some((Piece::Queen, col)) if col == enemy => return true,
                Some(_) => break,
                None => {}
            }
            r += dr;
            c += dc;
        }
    }

    // Check for attacking knights
    for (dr, dc) in [(-2,-1),(-2,1),(-1,-2),(-1,2),(1,-2),(1,2),(2,-1),(2,1)] {
        let (r, c) = (square.0 + dr, square.1 + dc);
        if (0..8).contains(&r) && (0..8).contains(&c) {
            if board[r as usize][c as usize] == Some((Piece::Knight, enemy)) {
                return true;
            }
        }
    }

    // Check for attacking pawns
    let pawn_dir: i8 = match color { Color::White => -1, Color::Black => 1 };
    for dc in [-1i8, 1] {
        let (r, c) = (square.0 + pawn_dir, square.1 + dc);
        if (0..8).contains(&r) && (0..8).contains(&c) {
            if board[r as usize][c as usize] == Some((Piece::Pawn, enemy)) {
                return true;
            }
        }
    }

    // Check for attacking king
    for (dr, dc) in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)] {
        let (r, c) = (square.0 + dr, square.1 + dc);
        if (0..8).contains(&r) && (0..8).contains(&c) {
            if board[r as usize][c as usize] == Some((Piece::King, enemy)) {
                return true;
            }
        }
    }

    false
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
    fn pinned_piece_cannot_move_away_from_king() {
        // White king on e1, white rook on e4, black rook on e8 — rook is pinned
        let mut board = empty_board();
        board[7][4] = Some((King, White));
        board[4][4] = Some((Rook, White));
        board[0][4] = Some((Rook, Black));
        let moves = legal_moves(&board, (4, 4));
        // Rook can only move along the pin file, not sideways
        assert!(moves.iter().all(|&(_, c)| c == 4));
    }

    #[test]
    fn must_block_check_or_move_king() {
        // White king on e1 in check from black rook on e8
        // White rook on a4 — only legal move is to block on e4 or king moves
        let mut board = empty_board();
        board[7][4] = Some((King, White));
        board[0][4] = Some((Rook, Black));
        board[4][0] = Some((Rook, White));
        let rook_moves = legal_moves(&board, (4, 0));
        // White rook must block on e-file
        assert!(rook_moves.iter().all(|&(_, c)| c == 4));
    }

    #[test]
    fn king_cannot_move_into_check() {
        // White king on e1, black rook on f8 — king cannot move to f-file
        let mut board = empty_board();
        board[7][4] = Some((King, White));
        board[0][5] = Some((Rook, Black));
        let moves = legal_moves(&board, (7, 4));
        assert!(moves.iter().all(|&(_, c)| c != 5));
    }

    #[test]
    fn is_in_check_detects_rook_attack() {
        let mut board = empty_board();
        board[7][4] = Some((King, White));
        board[0][4] = Some((Rook, Black));
        assert!(is_in_check(&board, White));
    }

    #[test]
    fn is_in_check_detects_knight_attack() {
        let mut board = empty_board();
        board[7][4] = Some((King, White));
        board[5][3] = Some((Knight, Black));
        assert!(is_in_check(&board, White));
    }

    #[test]
    fn is_not_in_check_when_blocked() {
        let mut board = empty_board();
        board[7][4] = Some((King, White));
        board[0][4] = Some((Rook, Black));
        board[4][4] = Some((Pawn, White)); // blocks the rook
        assert!(!is_in_check(&board, White));
    }
}
