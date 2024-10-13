use rusqlite::{Connection, Result};
use std::time::Instant;

fn main() -> Result<()> {
    // Open a connection to the SQLite database file
    let conn = Connection::open("biblia.db")?;

    // SQL query to fetch CONTENT from VERSE based on the specified conditions
    let mut stmt = conn.prepare(
        "SELECT
            NUMBER,
            CONTENT
        FROM VERSE
             WHERE BOOK_ID = ?1
               AND CHAPTER_NUMBER = ?2
               AND NUMBER >= ?3
               AND NUMBER <= ?4",
    )?;

    // Parameters for the query
    let book_id = 23;
    let chapter_number = 118;
    let number_start = 1;
    let number_end = 80;

    let start = Instant::now();
    // Execute the query and iterate over the results
    let content_iter = stmt.query_map(
        &[&book_id, &chapter_number, &number_start, &number_end],
        |row| {
            Ok(row.get::<_, String>(0)?) // CONTENT
        },
    )?;
    let duration = start.elapsed();
    println!("VERSE range read time elapsed: {:?}", duration);
    println!("Result Count: {:?}", content_iter.count());
    Ok(())
}
