use super::Table;
use sqlx::FromRow;
use sqlx::mysql::MySqlRow;

pub trait UserTrait{}
impl<'r,T> Table<'r, T>
    where T:FromRow<'r, MySqlRow>+UserTrait{
    pub fn public_user_fn(&self)->i32{1}
}
