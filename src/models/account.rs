use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

//模型代码

#[derive(FromRow,Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: u16,
    pub name: Option<String>
}
impl Account{
    pub fn show_name(&self)->String{
        return format!("{}-{}",self.name.as_ref().unwrap_or(&"".to_string()),self.id);
    }
}