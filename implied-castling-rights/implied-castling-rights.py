import chess
import csv
import sys

def maximum_castling_rights(board):
    return (
        (board.pieces_mask(chess.ROOK, chess.WHITE) & (chess.BB_A1 | chess.BB_H1) if board.king(chess.WHITE) == chess.E1 else chess.BB_EMPTY) |
        (board.pieces_mask(chess.ROOK, chess.BLACK) & (chess.BB_A8 | chess.BB_H8) if board.king(chess.BLACK) == chess.E8 else chess.BB_EMPTY)
    )

with open(sys.argv[1]) as f:
    for row in csv.reader(f):
        puzzle_id, fen, puzzle_moves, rating, deviation, popularity, played, themes, url = row[0:9]

        game_and_ply = url.removeprefix("https://lichess.org/")
        game, ply = game_and_ply.split("#")
        game = game.removesuffix("/black")
        ply = int(ply)

        board = chess.Board(fen)
        board.push_uci(puzzle_moves.split()[0])

        if board.castling_rights != maximum_castling_rights(board):
            print(",".join(row))
