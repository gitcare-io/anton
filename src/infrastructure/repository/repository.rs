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

pub trait EventStoreRepository {
    fn get_connection() -> PoolConnection { get(PoolType::EventStore) }
}

pub trait ReadRepository {
    fn get_connection() -> PoolConnection { get(PoolType::Read) }
}

pub fn __construct(pool_type: &'static str) -> Repository {
    match pool_type {
        "event_store" => Repository {
            conn: connection_manager::get(connection_manager::PoolType::EventStore)
        },
        _ => Repository {
            conn: connection_manager::get(connection_manager::PoolType::Read)
        },
    }
}