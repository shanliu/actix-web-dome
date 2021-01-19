use super::Table;

use sqlx::mysql::{MySqlDone};
use crate::models::account::Account;
use crate::models::Result;

impl<'c> Table<'c, Account> {

    pub async fn find_by_id(&self,user_id:u32)->Result<Account>{
        return sqlx::query_as::<_, Account>(
            r#"
            SELECT Fid as id,Frule_data as name from t_pm_valid_data_rule where Fid=?
        "#
        )
            .bind(user_id)
            .fetch_one(&*self.pool)
            .await;
    }
    // pub fn find_by_name(user_id:u32)->AccountModel{
    //
    //     let row = sqlx::query_as("SELECT Fid,Frule_data from t_pm_valid_data_rule where Fid>=?")
    //         .bind(info.id as u32)
    //         .fetch_one(&data.db).await;
    //     let brow:(u32,String)=row.map_err(|e|{
    //         match e {
    //             Error::Database(err) =>{
    //                 let b=err.message().to_string();
    //                 WebHandError{name:b}
    //             },
    //             Error::RowNotFound=>{
    //                 return WebHandError{name:"无记录".to_string()};
    //             }
    //             x@_ => {
    //                 let b:String=format!("{:?}", x);
    //                 return WebHandError{name:b};
    //             },
    //         }
    //     })?;
    // }


    pub async fn drop_table(&self) -> Result<MySqlDone> {
        sqlx::query("DROP TABLE IF EXISTS users;")
            .execute(&*self.pool)
            .await
    }

    pub async fn create_table(&self) -> Result<MySqlDone> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
            id VARCHAR(48) NOT NULL UNIQUE,
            name VARCHAR(64) NOT NULL UNIQUE,
            email VARCHAR(256) NOT NULL UNIQUE,
            PRIMARY KEY (id)
            )"#,
        )
            .execute(&*self.pool)
            .await
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Account> {
        sqlx::query_as(
            r#"
            SELECT `id`, `name`, `email`
            FROM `users`
            WHERE `id` = ?"#,
        )
            .bind(user_id)
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn add_user(&self, user: &Account) -> Result<MySqlDone> {
        sqlx::query(
            r#"
            INSERT INTO users (`id`, `name`, `email`)
            VALUES(?, ?, ?)"#,
        )
            .bind(&user.id)
            .bind(&user.name)
            .execute(&*self.pool)
            .await
    }

    pub async fn update_user(&self, user: &Account) -> Result<MySqlDone> {
        sqlx::query(
            r#"
            UPDATE users
            SET `name` = ?, `email` = ?
            WHERE `id` = ?
            "#,
        )
            .bind(&user.name)
            .bind(&user.id)
            .execute(&*self.pool)
            .await
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<MySqlDone> {
        sqlx::query(
            r#"
            DELETE FROM users
            WHERE `id` = ?
            "#,
        )
            .bind(user_id)
            .execute(&*self.pool)
            .await
    }
}
