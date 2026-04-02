mod board;
mod game;
mod moves;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::post,
    Json,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use uuid::Uuid;

use board::Color;
use game::GameState;
use moves::{legal_moves, game_status, GameStatus};

type GameId = String;
type AppState = Arc<Mutex<HashMap<GameId, GameState>>>;

// POST /game — create a new game, return its ID and initial board
#[derive(Serialize)]
struct NewGameResponse {
    id: GameId,
    board: Vec<Vec<Option<String>>>,
    turn: String,
}

async fn create_game(State(state): State<AppState>) -> Json<NewGameResponse> {
    let id = Uuid::new_v4().to_string();
    let game = GameState::new();
    let response = NewGameResponse {
        id: id.clone(),
        board: serialize_board(&game.board),
        turn: serialize_turn(game.turn),
    };
    state.lock().unwrap().insert(id, game);
    Json(response)
}

// POST /game/:id/legal-moves — return legal moves for a square
#[derive(Deserialize)]
struct LegalMovesRequest {
    from: [i8; 2],
}

#[derive(Serialize)]
struct LegalMovesResponse {
    moves: Vec<[i8; 2]>,
}

async fn handle_legal_moves(
    State(state): State<AppState>,
    Path(id): Path<GameId>,
    Json(req): Json<LegalMovesRequest>,
) -> Result<Json<LegalMovesResponse>, StatusCode> {
    let state = state.lock().unwrap();
    let game = state.get(&id).ok_or(StatusCode::NOT_FOUND)?;

    let moves = legal_moves(&game.board, (req.from[0], req.from[1]))
        .into_iter()
        .map(|(r, c)| [r, c])
        .collect();

    Ok(Json(LegalMovesResponse { moves }))
}

// POST /game/:id/move — apply a move, return updated board
#[derive(Deserialize)]
struct MoveRequest {
    from: [i8; 2],
    to: [i8; 2],
}

#[derive(Serialize)]
struct MoveResponse {
    board: Vec<Vec<Option<String>>>,
    turn: String,
    status: GameStatus,
}

async fn handle_move(
    State(state): State<AppState>,
    Path(id): Path<GameId>,
    Json(req): Json<MoveRequest>,
) -> Result<Json<MoveResponse>, StatusCode> {
    let mut state = state.lock().unwrap();
    let game = state.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    let (fr, fc) = (req.from[0], req.from[1]);
    let (tr, tc) = (req.to[0], req.to[1]);

    // Validate it's a legal move
    let legal = legal_moves(&game.board, (fr, fc));
    if !legal.contains(&(tr, tc)) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Apply the move
    game.board[tr as usize][tc as usize] = game.board[fr as usize][fc as usize];
    game.board[fr as usize][fc as usize] = None;
    game.turn = match game.turn {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let status = game_status(&game.board, game.turn);

    Ok(Json(MoveResponse {
        board: serialize_board(&game.board),
        turn: serialize_turn(game.turn),
        status,
    }))
}

fn serialize_board(board: &board::Board) -> Vec<Vec<Option<String>>> {
    board.iter().map(|row| {
        row.iter().map(|cell| {
            cell.map(|(piece, color)| {
                let c = match color { Color::White => 'w', Color::Black => 'b' };
                let p = match piece {
                    board::Piece::King   => 'K',
                    board::Piece::Queen  => 'Q',
                    board::Piece::Rook   => 'R',
                    board::Piece::Bishop => 'B',
                    board::Piece::Knight => 'N',
                    board::Piece::Pawn   => 'P',
                };
                format!("{}{}", c, p)
            })
        }).collect()
    }).collect()
}

fn serialize_turn(turn: Color) -> String {
    match turn {
        Color::White => "w".to_string(),
        Color::Black => "b".to_string(),
    }
}

#[tokio::main]
async fn main() {
    let state: AppState = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/game", post(create_game))
        .route("/game/{id}/legal-moves", post(handle_legal_moves))
        .route("/game/{id}/move", post(handle_move))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
