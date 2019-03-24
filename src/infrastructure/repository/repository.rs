use crate::infrastructure::repository::connection_manager::{get, PoolConnection, PoolType};

pub trait WriteRepository {
    fn get_connection() -> PoolConnection { get(PoolType::Write) }
}

pub trait ReadRepository {
    fn get_connection() -> PoolConnection { get(PoolType::Read) }
}
