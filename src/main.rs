use std::env;
use std::io;
use std::fs::File;
use std::str;

use pgn_reader::{BufferedReader, Visitor, RawHeader, Skip};
use shakmaty::{Chess, Position as _ };
use shakmaty::san::{SanPlus};
use shakmaty::uci::Uci;
use skytable::Connection;

struct Importer {
    con: Connection,
    pos: Chess,
    id: Option<String>,
    moves: Vec<Uci>,
}

impl Importer {
    fn new() -> io::Result<Importer> {
        Ok(Importer {
            con: Connection::new("127.0.0.1", 2003)?,
            id: None,
            pos: Chess::default(),
            moves: Vec::new(),
        })
    }
}

impl Visitor for Importer {
    type Result = ();

    fn begin_game(&mut self) {
        self.pos = Chess::default();
        self.moves.clear();
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        if key == b"Variant" {
            panic!("unexpected variant game")
        } else if key == b"FEN" {
            panic!("unexpected game from position")
        } else if key == b"Site" {
            self.id = str::from_utf8(value.as_bytes())
                .unwrap()
                .strip_prefix("https://lichess.org/")
                .map(|s| s.to_owned());
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn san(&mut self, san_plus: SanPlus) {
        if self.id.is_some() {
            if let Ok(m) = san_plus.san.to_move(&self.pos) {
                self.pos.play_unchecked(&m);
            } else {
                self.id = None;
            }
        }
    }

    fn end_game(&mut self) {
        if let Some(id) = self.id.take() {
            println!("{} {:?}", id, self.moves);
        }
    }
}

fn main() -> io::Result<()> {
    let mut importer = Importer::new()?;

    for arg in env::args().skip(1) {
        let file = File::open(&arg)?;
        println!("{}", arg);

        let uncompressed: Box<dyn io::Read> = if arg.ends_with(".bz2") {
            Box::new(bzip2::read::MultiBzDecoder::new(file))
        } else {
            Box::new(file)
        };

        let mut reader = BufferedReader::new(uncompressed);
        reader.read_all(&mut importer)?;
    }

    Ok(())
}
