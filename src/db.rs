use once_cell::sync::OnceCell;
use sqlx::postgres::PgPool;
use std::env;

pub static DATABASE_POOL: OnceCell<PgPool> = OnceCell::new();

pub async fn init_db_pool() {
    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("Database load failed.");
    DATABASE_POOL.set(pool).unwrap();
}

pub fn get_db_pool() -> &'static PgPool {
    unsafe { DATABASE_POOL.get_unchecked() }
}
