use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use crate::models::Model;

#[derive(FromRow,Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: u32,
    pub name: String
}
impl Model for Account{}
impl Account{
    pub fn show_name(&self)->String{
        return format!("{}-{}",self.name,self.id);
    }
}