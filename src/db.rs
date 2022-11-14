use once_cell::sync::OnceCell;
use sqlx::sqlite::SqlitePool;
use std::env;

pub static DATABASE_POOL: OnceCell<SqlitePool> = OnceCell::new();

pub async fn init_db_pool() {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap_or("sqlite:db.url".into()))
        .await
        .expect("Database load failed.");
    DATABASE_POOL.set(pool).unwrap();
}

pub fn get_db_pool() -> &'static SqlitePool {
    unsafe { DATABASE_POOL.get_unchecked() }
}
