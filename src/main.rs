use actix_files as fs;
use actix_web::{get, web, App, HttpServer, Result};
use shakmaty::{fen::Fen, CastlingMode, Chess, Position};

mod bot;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting test game @ http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(bot_move)
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/bot-move/{fen}")]
async fn bot_move(path: web::Path<String>) -> Result<String> {
    let data = path.into_inner();

    let fen: Fen = data
        .replace("%2F", "/")
        .replace("%20", " ")
        .parse()
        .unwrap();

    let position: Chess = fen.into_position(CastlingMode::Standard).unwrap();

    let bot_move = bot::next_move(&position);

    let new_position = position.play(&bot_move).unwrap();

    let fen_string = format!(
        "{}",
        Fen::from_position(new_position, shakmaty::EnPassantMode::Always)
    );
    Ok(fen_string)
}
