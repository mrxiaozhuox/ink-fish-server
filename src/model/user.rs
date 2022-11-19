use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{db::get_db_pool, error::Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreator {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserReader {
    pub id: i32,
    pub username: String,
    pub gender: String,
    pub email: String,
    pub reg_date: chrono::NaiveDate,
    pub introduction: String,
    pub avatar: String,
    pub role: i32,
}

pub struct User;
#[allow(dead_code)]
impl User {
    pub async fn get_user_by_id(id: i32) -> Result<UserReader> {
        let user = sqlx::query_as::<_, UserReader>("select * from user where id = $1;")
            .bind(id)
            .fetch_one(get_db_pool())
            .await?;
        Ok(user)
    }
    pub async fn get_user_by_email(email: String) -> Result<UserReader> {
        let user = sqlx::query_as::<_, UserReader>("select * from user where email = $1;")
            .bind(email)
            .fetch_one(get_db_pool())
            .await?;
        Ok(user)
    }

    pub fn gen_hashed_password(salt: String, pwd: String) -> String {
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
        format!("{:x}", digest)
    }

    pub async fn create_user(info: UserCreator) -> Result<UserReader> {
        let salt: String = std::iter::repeat_with(fastrand::alphanumeric)
            .take(12)
            .collect();

        let hashed_password = Self::gen_hashed_password(salt.clone(), info.password);

        let _ = sqlx::query(
            "insert into user (email, username, password, salt) values ($1, $2, $3, $4);",
        )
        .bind(info.email.clone())
        .bind(info.username)
        .bind(hashed_password)
        .bind(salt).execute(get_db_pool()).await?;

        Self::get_user_by_email(info.email).await
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Auth {
    id: String,
    user: i32,
    expire: i32,
}
