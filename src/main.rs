use std::env;
use std::io;
use std::fs::File;

use pgn_reader::{BufferedReader, Visitor};

struct Importer;

impl Visitor for Importer {
    type Result = ();

    fn end_game(&mut self) { }
}

fn main() -> Result<(), io::Error> {
    let mut importer = Importer;

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
