pub(crate) mod account;

pub(crate) type ModelResult<T> = sqlx::error::Result<T>;

pub(crate) trait Model{
    fn find(){

    }
    fn update(){

    }
    fn delete(){

    }
    fn insert(){

    }
}