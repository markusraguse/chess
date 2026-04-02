import { useState, useCallback, useEffect } from "react";

const API = "http://localhost:3000";

async function createGame(): Promise<{ id: string; board: Board; turn: string }> {
    const res = await fetch(`${API}/game`, { method: "POST" });
    return res.json();
}

async function fetchLegalMoves(gameId: string, from: [number, number]): Promise<[number, number][]> {
    const res = await fetch(`${API}/game/${gameId}/legal-moves`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ from }),
    });
    const data = await res.json();
    return data.moves;
}

type GameStatus =
    | { type: "Ongoing" }
    | { type: "Checkmate"; winner: "White" | "Black" }
    | { type: "Stalemate" };

async function sendMove(gameId: string, from: [number, number], to: [number, number]): Promise<{ board: Board; turn: string; status: GameStatus }> {
    const res = await fetch(`${API}/game/${gameId}/move`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ from, to }),
    });
    return res.json();
}

const PIECES: Record<string, string> = {
    wK: "♔", wQ: "♕", wR: "♖", wB: "♗", wN: "♘", wP: "♙",
    bK: "♚", bQ: "♛", bR: "♜", bB: "♝", bN: "♞", bP: "♟",
};

type PieceCode = keyof typeof PIECES | null;
type Board = PieceCode[][];
type Square = [number, number] | null;

const INITIAL_BOARD: Board = [
    ["bR","bN","bB","bQ","bK","bB","bN","bR"],
    ["bP","bP","bP","bP","bP","bP","bP","bP"],
    [null,null,null,null,null,null,null,null],
    [null,null,null,null,null,null,null,null],
    [null,null,null,null,null,null,null,null],
    [null,null,null,null,null,null,null,null],
    ["wP","wP","wP","wP","wP","wP","wP","wP"],
    ["wR","wN","wB","wQ","wK","wB","wN","wR"],
];

const FILES = ["a","b","c","d","e","f","g","h"];
const RANKS = ["8","7","6","5","4","3","2","1"];

export default function Board() {
    const [gameId, setGameId] = useState<string | null>(null);
    const [board, setBoard] = useState<Board>(INITIAL_BOARD.map(r => [...r]));
    const [selected, setSelected] = useState<Square>(null);
    const [legalSquares, setLegalSquares] = useState<[number, number][]>([]);
    const [turn, setTurn] = useState<"w" | "b">("w");
    const [lastMove, setLastMove] = useState<[Square, Square]>([null, null]);
    const [status, setStatus] = useState<string>("White to move");
    const [gameOver, setGameOver] = useState<GameStatus | null>(null);

    useEffect(() => {
        createGame().then(({ id, board, turn }) => {
            setGameId(id);
            setBoard(board);
            setTurn(turn as "w" | "b");
        });
    }, []);

    const handleSquareClick = useCallback(async (row: number, col: number) => {
        if (!gameId || gameOver) return;
        const piece = board[row][col];

        if (selected) {
            const [sr, sc] = selected;
            if (sr === row && sc === col) {
                setSelected(null);
                setLegalSquares([]);
                return;
            }
            const result = await sendMove(gameId, [sr, sc], [row, col]);
            setBoard(result.board);
            setTurn(result.turn as "w" | "b");
            setLastMove([selected, [row, col]]);
            setSelected(null);
            setLegalSquares([]);
            if (result.status.type === "Checkmate") {
                setGameOver(result.status);
            } else if (result.status.type === "Stalemate") {
                setGameOver(result.status);
            } else {
                setStatus(result.turn === "w" ? "White to move" : "Black to move");
            }
        } else {
            if (piece && piece[0] === turn) {
                setSelected([row, col]);
                const moves = await fetchLegalMoves(gameId, [row, col]);
                setLegalSquares(moves);
            }
        }
    }, [gameId, board, selected, turn, gameOver]);

    const isSelected = (r: number, c: number) =>
        selected !== null && selected[0] === r && selected[1] === c;

    const isLastMove = (r: number, c: number) =>
        (lastMove[0] !== null && lastMove[0][0] === r && lastMove[0][1] === c) ||
        (lastMove[1] !== null && lastMove[1][0] === r && lastMove[1][1] === c);

    const isLegal = (r: number, c: number) =>
        legalSquares.some(([lr, lc]) => lr === r && lc === c);

    const isLight = (r: number, c: number) => (r + c) % 2 === 0;

    return (
        <div style={{
            minHeight: "100vh",
            background: "#1a1008",
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            justifyContent: "center",
            fontFamily: "'Georgia', serif",
            padding: "2rem",
        }}>

            {/* Title */}
            <div style={{
                marginBottom: "1.5rem",
                textAlign: "center",
            }}>
                <h1 style={{
                    color: "#c9a84c",
                    fontSize: "1.4rem",
                    fontWeight: "400",
                    letterSpacing: "0.25em",
                    textTransform: "uppercase",
                    margin: 0,
                }}>Chess Engine</h1>
                <p style={{
                    color: gameOver ? "#c9a84c" : "#6b5a3a",
                    fontSize: "0.75rem",
                    letterSpacing: "0.15em",
                    margin: "0.3rem 0 0",
                    textTransform: "uppercase",
                }}>
                    {gameOver
                        ? gameOver.type === "Checkmate"
                            ? `${gameOver.winner} wins`
                            : "Stalemate — draw"
                        : status}
                </p>
            </div>

            {/* Board wrapper */}
            <div style={{
                display: "flex",
                alignItems: "center",
                gap: "0.5rem",
            }}>

                {/* Rank labels left */}
                <div style={{ display: "flex", flexDirection: "column" }}>
                    {RANKS.map(r => (
                        <div key={r} style={{
                            width: "16px",
                            height: "64px",
                            display: "flex",
                            alignItems: "center",
                            justifyContent: "center",
                            color: "#6b5a3a",
                            fontSize: "0.7rem",
                            letterSpacing: "0.05em",
                        }}>{r}</div>
                    ))}
                </div>

                <div>
                    {/* Board grid */}
                    <div style={{
                        display: "grid",
                        gridTemplateColumns: "repeat(8, 64px)",
                        gridTemplateRows: "repeat(8, 64px)",
                        border: "2px solid #3d2e14",
                        boxShadow: "0 0 60px rgba(0,0,0,0.8), inset 0 0 30px rgba(0,0,0,0.3)",
                    }}>
                        {board.map((row, r) =>
                                row.map((piece, c) => {
                                    const light = isLight(r, c);
                                    const sel = isSelected(r, c);
                                    const lm = isLastMove(r, c);
                                    const legal = isLegal(r, c);

                                    let bg = light ? "#f0d9b5" : "#b58863";
                                    if (lm) bg = light ? "#cdd16f" : "#aaa23a";
                                    if (legal) bg = light ? "#c8e6c9" : "#81c784";
                                    if (sel) bg = light ? "#f6f669" : "#baca2b";

                                    return (
                                        <div
                                            key={`${r}-${c}`}
                                            onClick={() => handleSquareClick(r, c)}
                                            style={{
                                                width: "64px",
                                                height: "64px",
                                                background: bg,
                                                display: "flex",
                                                alignItems: "center",
                                                justifyContent: "center",
                                                cursor: "pointer",
                                                position: "relative",
                                                transition: "background 0.1s",
                                            }}
                                        >
                                            {piece && (
                                                <span style={{
                                                    fontSize: "42px",
                                                    lineHeight: 1,
                                                    userSelect: "none",
                                                    filter: piece[0] === "w"
                                                        ? "drop-shadow(0 1px 1px rgba(0,0,0,0.4))"
                                                        : "drop-shadow(0 1px 1px rgba(0,0,0,0.5))",
                                                    transform: sel ? "scale(1.15)" : "scale(1)",
                                                    transition: "transform 0.1s",
                                                    color: piece[0] === "w" ? "#fff" : "#1a1008",
                                                    WebkitTextStroke: piece[0] === "w" ? "0.5px #555" : "0.5px #000",
                                                }}>
                        {PIECES[piece]}
                      </span>
                                            )}
                                        </div>
                                    );
                                })
                        )}
                    </div>

                    {/* File labels bottom */}
                    <div style={{ display: "flex", marginTop: "4px" }}>
                        {FILES.map(f => (
                            <div key={f} style={{
                                width: "64px",
                                height: "16px",
                                display: "flex",
                                alignItems: "center",
                                justifyContent: "center",
                                color: "#6b5a3a",
                                fontSize: "0.7rem",
                                letterSpacing: "0.05em",
                            }}>{f}</div>
                        ))}
                    </div>
                </div>
            </div>

            {/* Turn indicator */}
            <div style={{
                marginTop: "1.5rem",
                display: "flex",
                alignItems: "center",
                gap: "10px",
            }}>
                <div style={{
                    width: "12px",
                    height: "12px",
                    borderRadius: "50%",
                    background: turn === "w" ? "#f0d9b5" : "#1a1008",
                    border: "1.5px solid #c9a84c",
                    boxShadow: turn === "w" ? "0 0 8px rgba(240,217,181,0.6)" : "none",
                    transition: "all 0.3s",
                }}/>
                <span style={{
                    color: "#6b5a3a",
                    fontSize: "0.7rem",
                    letterSpacing: "0.15em",
                    textTransform: "uppercase",
                }}>{turn === "w" ? "White" : "Black"}</span>
            </div>

            {/* Reset */}
            <button
                onClick={() => {
                    createGame().then(({ id, board, turn }) => {
                        setGameId(id);
                        setBoard(board);
                        setTurn(turn as "w" | "b");
                    });
                    setSelected(null);
                    setLegalSquares([]);
                    setLastMove([null, null]);
                    setStatus("White to move");
                    setGameOver(null);
                }}
                style={{
                    marginTop: "1.2rem",
                    background: "transparent",
                    border: "1px solid #3d2e14",
                    color: "#6b5a3a",
                    padding: "0.4rem 1.2rem",
                    fontSize: "0.7rem",
                    letterSpacing: "0.2em",
                    textTransform: "uppercase",
                    cursor: "pointer",
                    transition: "all 0.2s",
                    fontFamily: "inherit",
                }}
                onMouseEnter={e => {
                    (e.target as HTMLButtonElement).style.borderColor = "#c9a84c";
                    (e.target as HTMLButtonElement).style.color = "#c9a84c";
                }}
                onMouseLeave={e => {
                    (e.target as HTMLButtonElement).style.borderColor = "#3d2e14";
                    (e.target as HTMLButtonElement).style.color = "#6b5a3a";
                }}
            >
                New Game
            </button>
        </div>
    );
}
