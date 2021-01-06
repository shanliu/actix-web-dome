use crate::models::account::AccountModel;
use sqlx::{MySql, Pool};
use crate::models::ModelResult;

pub struct AccountSerice<'a>{
    db:&'a Pool<MySql>
}
impl<'a> AccountSerice<'a>{
    pub fn new(db:&'a Pool<MySql>)->Self{
        return AccountSerice{
            db:db
        };
    }
    pub async fn find_by_id(&self,user_id:u32)->ModelResult<AccountModel>{
        return sqlx::query_as::<_, AccountModel>(
            r#"
            SELECT Fid as id,Frule_data as name from t_pm_valid_data_rule where Fid=?
        "#
        )
            .bind(user_id)
            .fetch_one(self.db)
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
}