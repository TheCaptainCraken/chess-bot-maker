use actix_files as fs;
use actix_web::{get, middleware, web, App, HttpServer, Result};
use rand::seq::SliceRandom;
use shakmaty::{fen::Fen, CastlingMode, Chess, Position};

struct GameState {
    position: Chess,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    log::info!("starting HTTP server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(GameState {
                position: Chess::default(),
            }))
            .wrap(middleware::Logger::default())
            .service(bot_move)
            .service(health_check)
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/bot-move/{fen}")] // <- define path parameters
async fn bot_move(path: web::Path<String>) -> Result<String> {
    let data = path.into_inner();

    let fen: Fen = data
        .replace("%2F", "/")
        .replace("%20", " ")
        .parse()
        .unwrap();

    log::info!("fen: {}", fen);

    let position: Chess = fen.into_position(CastlingMode::Standard).unwrap();

    let legal_moves = position.legal_moves();

    let bot_move = legal_moves.choose(&mut rand::thread_rng()).unwrap();

    let new_position = position.play(&bot_move).unwrap();

    Ok(format!(
        "{}",
        Fen::from_position(new_position, shakmaty::EnPassantMode::Always)
    ))
}

#[get("/health-check")]
async fn health_check() -> Result<String> {
    Ok("HealthCheck performed correctly".to_string())
}
