pub mod models;
pub mod schema;

use bytes::Bytes;
use futures_util::StreamExt;

use axum::{extract::BodyStream, routing::get, routing::post, Router};
use tokio::net::TcpListener;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(help))
        .route("/echo", post(echo))
        .route("/post", get(get_posts));

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

async fn help() -> &'static str {
    "Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`\n"
}

async fn echo(mut stream: BodyStream) -> Bytes {
    if let Some(Ok(s)) = stream.next().await {
        s
    } else {
        Bytes::new()
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn get_posts() -> Result<String, String> {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(models::Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    let mut result = String::new();

    println!("Displaying {} posts", results.len());
    for post in results {
        result.push_str(&format!("{}\n", post.title));
        result.push_str("-----------\n");
        result.push_str(&format!("{}", post.body));
        result.push('\n');
    }

    Ok(result)
}
