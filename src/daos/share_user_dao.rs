use super::Table;
use sqlx::FromRow;
use sqlx::mysql::MySqlRow;

// 某些模块的公共实现
pub trait UserTrait{}

impl<'r,T> Table<'r, T>
    where T:FromRow<'r, MySqlRow>+UserTrait{
    pub fn public_user_fn(&self)->i32{1}
}
