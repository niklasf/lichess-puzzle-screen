use std::env;
use std::io;
use std::str;

use serde::Deserialize;
use rocksdb::DB;

#[derive(Debug, Deserialize)]
struct Puzzle {
    id: String,
    fen: String,
    moves: String,
    rating: i32,
    rd: i32,
    popularity: i32,
    played: u32,
    themes: String,
    url: String,
}

fn main() -> io::Result<()> {
    let db = DB::open_for_read_only(&rocksdb::Options::default(), "/scratch", true).expect("rocksdb");
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(env::args().nth(1).expect("csv as argument"))?;

    for result in reader.deserialize() {
        let puzzle: Puzzle = result?;

        let mut game_and_ply = puzzle.url.strip_prefix("https://lichess.org/").expect("game id").splitn(2, "#");
        let game = game_and_ply.next().expect("game");
        let game = game.strip_suffix("/black").unwrap_or(game);
        let _ply: u32 = game_and_ply.next().expect("ply").parse().unwrap();

        let game_moves = db.get(game).expect("rocksdb get").map(|val| String::from_utf8(val).expect("unicode"));

        if let Some(game_moves) = game_moves {
            println!("{},{},{},{},{},{},{},{},{},{}", puzzle.id, puzzle.fen, puzzle.moves, puzzle.rating, puzzle.rd, puzzle.popularity, puzzle.played, puzzle.themes, puzzle.url, game_moves);
        } else {
            eprintln!("Did not find game moves for {}", game);
        }
    }

    Ok(())
}
