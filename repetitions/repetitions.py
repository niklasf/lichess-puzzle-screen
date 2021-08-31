import chess
import csv
import sys

with open(sys.argv[1]) as f:
    for row in csv.reader(f):
        puzzle_id, fen, puzzle_moves, rating, deviation, popularity, played, themes, url, game_moves = row

        game_and_ply = url.removeprefix("https://lichess.org/")
        game, ply = game_and_ply.split("#")
        game = game.removesuffix("/black")
        ply = int(ply)

        board = chess.Board()
        for uci in game_moves.split()[:ply]:
            board.push_uci(uci)

        while board.move_stack:
            if board.is_repetition(2):
                print(",".join(row))
                break

            move = board.pop()
            if board.is_irreversible(move):
                break
