use redb::{Database, TableDefinition};

use std::error::Error;
use std::time::Instant;

const VERSE: TableDefinition<(u8, u8, u8), &str> = TableDefinition::new("VERSE");

fn main() -> Result<(), Box<dyn Error>> {
    let db = Database::create("biblia.redb")?;
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(VERSE)?;
    let start = Instant::now();
    let range = table.range((23, 118, 1)..(23, 118, 80));
    let duration = start.elapsed();
    println!("VERSE range read time elapsed: {:?}", duration);
    println!("VERSE range count: {:?}", range.iter().count());
    Ok(())
}
