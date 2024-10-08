use redb::{Database, TableDefinition};

use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

const VERSES: TableDefinition<(u8, u8, u8), &str> = TableDefinition::new("VERSES");

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open("db/VERSES.csv")?;
    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);
    let db = Database::create("db/biblia.redb")?;
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(VERSES)?;
        for result in rdr.records() {
            let record = result?;
            let book: u8 = record[0].parse().unwrap();
            let chapter: u8 = record[1].parse().unwrap();
            let verse: u8 = record[2].parse().unwrap();
            table.insert((book, chapter, verse), &record[3])?;
        }
    }

    write_txn.commit()?;

    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(VERSES)?;
    println!("{}", table.get((1, 1, 1))?.unwrap().value());

    Ok(())
}
