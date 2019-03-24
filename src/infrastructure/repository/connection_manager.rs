use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;

lazy_static! {
    pub static ref POOL_WRITE: Pool<ConnectionManager<PgConnection>> = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(
            env::var("DATABASE_URL_WRITE").expect("DATABASE_URL_WRITE must be set")
        ))
        .expect("Failed to create pool write.");
    pub static ref POOL_READ: Pool<ConnectionManager<PgConnection>> = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(
            env::var("DATABASE_URL_READ").expect("DATABASE_URL_READ must be set")
        ))
        .expect("Failed to create pool read.");
}

pub enum PoolType {
    Read,
    Write,
}

pub type PoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get(pool_type: PoolType) -> PoolConnection {
    match pool_type {
        PoolType::Read => POOL_READ.get().expect("Failed to get pooled connection"),
        PoolType::Write => POOL_WRITE.get().expect("Failed to get pooled connection"),
    }
}
