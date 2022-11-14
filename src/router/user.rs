use salvo::prelude::*;

use crate::error::Result;

#[handler]
pub fn get_account(req: &mut Request) -> Result<&'static str> {
    
    let user_id = req.param::<u32>("id");
    
    Ok("1231231231231")
}

pub fn router() -> Router {
    Router::with_path("user")
    .push(
        Router::with_path("<id>")
        .get(get_account)
    )
}