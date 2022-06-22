use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub table: String,
    #[serde(default = "crate::sudoku_table::Format::default")]
    pub output_format: crate::sudoku_table::Format,
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Ok { table: String },
    Error { msg: String },
}
