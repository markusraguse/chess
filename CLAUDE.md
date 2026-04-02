# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A chess engine with a Rust/Axum backend and React/TypeScript frontend. The backend owns all game state and move validation. The frontend is a pure rendering layer that communicates with the backend via REST.

## Build & Development Commands

### Both at once (from repo root)
```bash
npm run dev        # Starts backend (port 3000) and frontend (Vite HMR) concurrently
```

### Backend (Rust)
```bash
cd backend
cargo run          # Starts HTTP server on 0.0.0.0:3000
cargo test
cargo test <name>  # Run a specific test
cargo test -- --nocapture  # Show println! output
```

### Frontend (React + Vite)
```bash
cd frontend
npm run dev        # Dev server with HMR
npm run build      # Type-check + production build
npm run lint       # ESLint
```

## Architecture

### Workspace Layout
- `backend/` — Rust crate (Axum HTTP server, Rust edition 2024)
- `frontend/` — React 19 + TypeScript, bundled with Vite

### Backend (`backend/src/`)
- `main.rs` — Axum router, shared `AppState` (`Arc<Mutex<HashMap<GameId, GameState>>>`), and all HTTP handlers. Three endpoints:
  - `POST /game` — creates a new game, returns `{ id, board, turn }`
  - `POST /game/:id/legal-moves` — returns legal destination squares for a given piece
  - `POST /game/:id/move` — validates and applies a move, returns updated `{ board, turn }`
- `board.rs` — Core types: `Piece` enum, `Color` enum, `Board` type alias (`[[Option<(Piece, Color)>; 8]; 8]`)
- `game.rs` — `GameState` struct (board + turn) and `initial_board()`
- `moves/mod.rs` — `legal_moves(board, from)` dispatcher; routes to the correct piece module
- `moves/knight.rs` — `knight_moves()` implementation and its unit tests

### Move validation structure
Each piece gets its own file under `moves/`. To add a new piece:
1. Create `moves/<piece>.rs` with a `<piece>_moves(board, from, color) -> Vec<(i8, i8)>` function and tests
2. Add `mod <piece>; pub use <piece>::<piece>_moves;` to `moves/mod.rs`
3. Add a match arm in `legal_moves()` in `moves/mod.rs`

### Frontend (`frontend/src/`)
- `Board.tsx` — All UI and API calls. On mount it calls `POST /game` to create a session. Board and turn state come entirely from backend responses — the frontend holds no authoritative game state.
  - `PieceCode` string format (`"wN"`, `"bP"`, etc.) is used for display and sent/received from the backend as-is

### Current Gaps
- Only knight move validation is implemented — all other pieces return no legal moves
- Check, checkmate, castling, en passant, and promotion are not yet implemented
- Game state is in-memory only — restarting the backend loses all games
