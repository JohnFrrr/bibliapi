use axum::{
    extract::BodyStream,
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use bytes::Bytes;
use futures_util::StreamExt;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tokio::net::TcpListener;

// New struct to wrap the response data
#[derive(Serialize, Deserialize, Debug)]
struct ResponseWrapper<T> {
    data: T,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Verse {
    n: u8,
    txt: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ChapterVerses {
    book_number: u8,
    chapter_number: u8,
    verses: Vec<Verse>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route(
            "/book/:book_number/chapter/:chapter_number",
            get(get_chapter_verses),
        )
        .route("/echo", post(echo));

    // run it
    let addr = "0.0.0.0:8080";
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::Server::from_tcp(tcp_listener.into_std().unwrap())
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_chapter_verses(
    Path((book_number, chapter_number)): Path<(u8, u8)>,
) -> Json<ResponseWrapper<ChapterVerses>> {
    let connection = match Connection::open("./db/biblia.db") {
        Ok(conn) => conn,
        Err(_) => panic!("Failed to connect to the database"),
    };

    let mut statement = match connection
        .prepare("SELECT NUMBER, CONTENT FROM VERSE WHERE BOOK_ID = ?1 AND CHAPTER_NUMBER = ?2")
    {
        Ok(stmt) => stmt,
        Err(_) => panic!("Failed to prepare the statement"),
    };

    let verse_iter = statement
        .query_map(params![book_number, chapter_number], |row| {
            Ok(Verse {
                n: row.get(0)?,
                txt: row.get(1)?,
            })
        })
        .expect("Failed to query and map verses");

    let verses: Vec<Verse> = verse_iter.filter_map(Result::ok).collect();

    let chapter_verses = ChapterVerses {
        book_number,
        chapter_number,
        verses,
    };

    Json(ResponseWrapper {
        data: chapter_verses,
    })
}

async fn echo(mut stream: BodyStream) -> Bytes {
    if let Some(Ok(s)) = stream.next().await {
        s
    } else {
        Bytes::new()
    }
}
