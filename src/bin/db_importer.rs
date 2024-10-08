use redb::{Database, ReadableTable, TableDefinition};

use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

const VERSE: TableDefinition<(u8, u8, u8), &str> = TableDefinition::new("VERSE");
const BOOK: TableDefinition<u8, &str> = TableDefinition::new("BOOK");

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open("db/csv/VERSES.csv")?;
    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);
    let db = Database::create("db/redb/biblia.redb")?;
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(VERSE)?;
        let start = Instant::now();
        for result in rdr.records() {
            let record = result?;
            let book: u8 = record[0].parse().unwrap();
            let chapter: u8 = record[1].parse().unwrap();
            let verse: u8 = record[2].parse().unwrap();
            table.insert((book, chapter, verse), &record[3])?;
        }
        let duration = start.elapsed();
        println!("VERSE write time elapsed: {:?}", duration);
    }

    write_txn.commit()?;

    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(VERSE)?;
    println!("First verse: {}", table.first()?.unwrap().1.value());
    println!("Last verse: {}", table.last()?.unwrap().1.value());

    let file = File::open("db/csv/BOOKS.csv")?;
    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(BOOK)?;
        let start = Instant::now();
        for result in rdr.records() {
            let record = result?;
            let book: u8 = record[0].parse().unwrap();
            table.insert(book, &record[1])?;
        }
        let duration = start.elapsed();
        println!("BOOK write time elapsed: {:?}", duration);
    }

    write_txn.commit()?;

    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(BOOK)?;
    println!("First book: {}", table.first()?.unwrap().1.value());
    println!("Last book: {}", table.last()?.unwrap().1.value());
    Ok(())
}
