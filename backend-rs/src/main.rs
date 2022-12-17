mod handlers;
mod protocol;
mod recursive_solver;
mod sudoku_table;

use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info"),
    )
    .init();
    let addr = "0.0.0.0:5000";
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .route(
                "/recursive_solver",
                actix_web::web::post().to(recursive_solver_handler),
            )
            .route(
                "/healthy",
                actix_web::web::get().to(|| actix_web::HttpResponse::Ok()),
            )
    })
    .bind(addr)?
    .run()
    .await
}
