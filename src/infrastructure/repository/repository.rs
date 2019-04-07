use crate::infrastructure::repository::connection_manager::{get, PoolConnection, PoolType};
use crate::infrastructure::repository::connection_manager;

pub struct Repository {
    conn: PoolConnection,
}

pub trait CommonRepository {
    fn conn(&self) -> &PoolConnection;
}

impl CommonRepository for Repository {
    fn conn(&self) -> &PoolConnection {
        &self.conn
    }
}

pub trait WriteRepository {
    fn get_connection() -> PoolConnection { get(PoolType::Write) }
}

pub trait ReadRepository {
    fn get_connection() -> PoolConnection { get(PoolType::Read) }
}

pub fn __construct() -> Repository {
    Repository {
        conn: connection_manager::get(connection_manager::PoolType::Write)
    }
}