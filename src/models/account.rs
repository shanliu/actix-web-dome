use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

#[derive(FromRow,Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: u32,
    pub name: String
}
impl Account{
    pub fn show_name(&self)->String{
        return format!("{}-{}",self.name,self.id);
    }
}