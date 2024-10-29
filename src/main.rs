use actix_files as fs;
use actix_web::{get, http::header::ContentType, middleware, web, App, HttpResponse, HttpServer};
use shakmaty::{Chess, Position};

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
            .route("/gameOver", web::get().to(game_over))
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn game_over(data: web::Data<GameState>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(format!(
            "{{\"gameOver\": {}}}",
            data.position.is_game_over()
        ))
}
