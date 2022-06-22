use crate::recursive_solver::RecursiveSolver;
use crate::sudoku_table::SudokuTable;

fn create_error(
    msg: String,
) -> actix_web::Result<actix_web::web::Json<crate::protocol::Response>> {
    return Ok(actix_web::web::Json(crate::protocol::Response::Error {
        msg,
    }));
}

pub async fn recursive_solver_handler(
    request: actix_web::web::Json<crate::protocol::Request>,
) -> actix_web::Result<actix_web::web::Json<crate::protocol::Response>> {
    let mut t = SudokuTable::default();
    if !t.try_parse(&request.table) {
        return create_error("Cannot parse input table".to_string());
    }

    let mut s = RecursiveSolver::default();
    s.set_table(t);
    let result = match s.solve() {
        Ok(solved_table) => solved_table,
        Err(e) => {
            return create_error(e.to_string());
        }
    };
    return Ok(actix_web::web::Json(crate::protocol::Response::Ok {
        table: result.to_string(request.output_format),
    }));
}
