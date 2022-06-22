mod protocol;
mod recursive_solver;
mod sudoku_table;

use recursive_solver::RecursiveSolver;
use sudoku_table::{Format, SudokuTable};

async fn recursive_solver_handler(
    request: actix_web::web::Json<protocol::Request>,
) -> actix_web::Result<actix_web::web::Json<protocol::Response>> {
    Ok(actix_web::web::Json(protocol::Response::Ok {
        table: request.table.clone(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let addr = "127.0.0.1:5000";
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .route(
                "/recursive_solver",
                actix_web::web::post().to(recursive_solver_handler),
            )
            .route(
                "/",
                actix_web::web::get().to(|| actix_web::HttpResponse::Ok()),
            )
    })
    .bind(addr)?
    .run()
    .await
}

const EXAMPLE2: &str = "
    . . .  . . .  . . .
    . . .  . . 3  . 8 5
    . . 1  2 . .  . . .

    . . .  5 . 7  . . .
    . . 4  . . .  1 . .
    . 9 .  . . .  . . .

    5 . .  . . .  . 7 3
    . . 2  . 1 .  . . .
    . . .  . 4 .  . . 9
    ";
fn main2() {
    let mut t: SudokuTable = Default::default();
    assert!(t.try_parse(EXAMPLE2));
    println!("{}", t.to_string(Format::Indent));
    let mut s = RecursiveSolver::default();
    s.set_table(t);
    let result = s.solve().unwrap();
    println!("{}", result.to_string(Format::Indent));
}
