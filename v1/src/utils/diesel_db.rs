use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::sync::Arc;

pub type DieselPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnPool = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_diesel_pool() -> Arc<DieselPool> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);

    Arc::new(
        r2d2::Pool::builder()
            .build(manager)
            .expect("failed init diesel pool."),
    )
}
