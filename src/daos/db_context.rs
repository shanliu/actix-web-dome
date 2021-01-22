use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySqlPool, Pool, MySql};
use std::sync::Arc;
use crate::models::account::Account;



pub struct Table<'r,T>
    where
        T: FromRow<'r, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _marker:std::marker::PhantomData<&'r T>
}

impl<'r, T> Table<'r,T>
    where
        T: FromRow<'r, MySqlRow>,
{
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _marker:std::marker::PhantomData::default()
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