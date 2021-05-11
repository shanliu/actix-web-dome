use super::Table;
use sqlx::mysql::{MySqlQueryResult, MySqlRow};
use crate::models::account::Account;
use crate::models::Result;
use sqlx::Row;

impl<'c> Table<'c, Account> {

    pub async fn find_by_id(&self,user_id:u32)->Result<Account>{
        return sqlx::query_as::<_, Account>(
            r#"
                SELECT Fid as id,Fserial_no as name from t_pm_product where Fserial_no=?
            "#
        )
        .bind("ZY0101191223000008")
        .fetch_one(&*self.pool)
        .await;

    }
    pub async fn test(&self){

        #[derive(sqlx::Type,Clone,Debug)]
        #[sqlx(transparent)]
        pub struct MyInt4(i32);



        // let a=sqlx::query_as!(Account,r#"
        //     SELECT id,if(customer_surname is null,'',customer_surname) as "name!" from orders_list
        // "#)
        //     .fetch_one(&*self.pool)
        //     .await
        //     .unwrap();

        let a=sqlx::query_as::<_, Account>(
            r#"
            SELECT id,customer_surname as name from orders_list where id=?
        "#
        )
            .bind(3)
            .fetch_one(&*self.pool)
            .await;

        //
        // use sql_builder::SqlBuilder;
        //
        // let sql = SqlBuilder::select_from("orders_list")
        //     .field("id")
        //     .field("customer_surname as name")
        //     .and_where_eq("id",1)
        //     .sql().unwrap();
        // let a=sqlx::query_as::<_, Account>(sql.as_str())
        //     .fetch_one(&pool)
        //     .await
        //     .unwrap();
        // println!("{:?}",a);


        //
        // let account = sqlx::query!(r#"select id as "id?",customer_surname as "name!" from orders_list"#)
        //     .fetch_one(&*self.pool)
        //     .await;
        //
        // println!("{:?}",(account.unwrap().id as Option<i32>).unwrap_or(0));

        let account = sqlx::query(r#"select customer_surname as "name!" from orders_list limit 100"#)
            .try_map(|row:MySqlRow|{
                return Ok(row.try_get::<::std::option::Option<String>, _>(0usize));
            })
            .fetch_all(&*self.pool)
            .await.unwrap();

        println!("{}",account.len());
        for a  in account {
            println!("{:?}",a);
        }

        // println!("{:?}",account.try_get::<::std::option::Option<String>, _>(0usize));
        //
        // let account = sqlx::query!("select id,name from orders_list")
        //     .fetch_one(&*self.pool)
        //     .await;
        // println!("{:?}",account);

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

    pub async fn get_user_by_id(&self, user_id: &str)->Result<Account> {

        // let a=sqlx::query!(
        //         r#"
        //            SELECT id,customer_surname from orders_list where id=? and 1
        //         "#,
        //         user_id.parse::<i32>().unwrap()
        //     )
        //     .fetch_one(&*self.pool)
        //     .await?;
        //
        // Result::Ok(Account{
        //     id: a.id as u32,
        //     name: (a.customer_surname as Option<String>).unwrap_or("".to_string())
        // })
        Result::Ok(Account{
            id: 1,
            name:"".to_string()
        })
    }
    #[allow(dead_code)]
    pub async fn add_user(&self, user: &Account) -> Result<MySqlQueryResult> {
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
    #[allow(dead_code)]
    pub async fn update_user(&self, user: &Account) -> Result<MySqlQueryResult> {
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
    #[allow(dead_code)]
    pub async fn delete_user(&self, user_id: &str) -> Result<MySqlQueryResult> {
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
