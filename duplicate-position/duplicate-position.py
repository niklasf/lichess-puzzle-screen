import chess
import csv
import sys

def link(puzzle_id):
    return f"https://lichess.org/training/{puzzle_id}"

with open(sys.argv[1]) as f:
    first = {}

    for row in csv.reader(f):
        puzzle_id, fen, puzzle_moves, rating, deviation, popularity, played, themes, url = row[0:9]

        board = chess.Board(fen)
        board.push_uci(puzzle_moves.split()[0])
        epd = board.epd()

        if epd in first:
            print(f"{link(puzzle_id)} {link(first[epd])}")
        else:
            first[epd] = puzzle_id
