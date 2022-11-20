use salvo::prelude::*;

use crate::{
    error::{AppError, Result},
    model::user::{User, UserReader},
};

#[handler]
pub async fn get_account(req: &mut Request, _resp: &mut Response) -> Result<Json<UserReader>> {

    let user_id = req.query::<i32>("id");
    let user_email = req.query::<String>("email");


    let user = if let Some(id) = user_id {
        User::get_user_by_id(id).await?
    } else if let Some(email) = user_email {
        User::get_user_by_email(email).await?
    } else {
        return Err(AppError::MissingParam("ID or Email"));
    };

    Ok(Json(user))
}

pub fn router() -> Router {
    Router::with_path("user").push(Router::with_path("info").get(get_account))
}
