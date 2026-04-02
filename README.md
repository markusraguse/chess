# Chess Engine

A chess engine with a Rust backend and React frontend. The backend owns all game state and enforces move legality — the frontend is a pure rendering layer.

## Stack

- **Backend** — Rust, Axum, Tokio
- **Frontend** — React 19, TypeScript, Vite

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/)

### Run both together

```bash
npm install
npm run dev
```

Frontend runs on `http://localhost:5173`, backend on `http://localhost:3000`.

### Run separately

```bash
# Backend
cd backend
cargo run

# Frontend
cd frontend
npm install
npm run dev
```

## Features

- Legal move highlighting for all pieces
- Move validation enforced server-side
- Check, checkmate, and stalemate detection
- Game state stored per session via a game ID

## Project Structure

```
chess-engine/
├── backend/
│   └── src/
│       ├── main.rs         # Axum server, HTTP handlers, shared state
│       ├── board.rs        # Piece and Color types
│       ├── game.rs         # GameState, initial board setup
│       └── moves/          # Move validation per piece
│           ├── mod.rs      # legal_moves() dispatcher, check/stalemate detection
│           ├── pawn.rs
│           ├── knight.rs
│           ├── bishop.rs
│           ├── rook.rs
│           ├── queen.rs
│           └── king.rs
└── frontend/
    └── src/
        ├── main.tsx
        ├── App.tsx
        └── Board.tsx       # Board UI, all API calls
```

## API

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/game` | Create a new game, returns `{ id, board, turn }` |
| `POST` | `/game/:id/legal-moves` | Get legal moves for a square, body: `{ from: [r, c] }` |
| `POST` | `/game/:id/move` | Apply a move, body: `{ from: [r, c], to: [r, c] }` |

## Running Tests

```bash
cd backend
cargo test
```
