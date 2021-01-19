use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySqlPool, Pool, MySql};
use std::sync::Arc;
use crate::models::account::Account;



pub struct Table<'r,T>
    where
        T: FromRow<'r, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: fn(&'r MySqlRow) -> Result<T, sqlx::Error>
}

impl<'r, T> Table<'r,T>
    where
        T: FromRow<'r, MySqlRow>,
{
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row
        }
    }
}


pub struct Database<'r> {
    pub users: Arc<Table<'r,Account>>,
}

impl<'r> Database<'r> {
    pub async fn new(poll:Pool<MySql>) -> Database<'r>{
        Database {
            users: Arc::from(Table::new(Arc::from(poll.clone()))),
        }
    }
}