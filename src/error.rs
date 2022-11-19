use salvo::{async_trait, Depot, Request, Response, Writer, writer::Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug)]
struct ErrorResponse {
    detail: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("sqlx: `{0}`")]
    Sqlx(#[from] sqlx::error::Error)
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let resp = ErrorResponse { detail: self.to_string() };
        res.render(Json(resp));
    }
}

pub type Result<T> = core::result::Result<T, AppError>;