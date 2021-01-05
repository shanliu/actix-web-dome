use crate::model::Model;
#[derive(sqlx::FromRow)]
pub struct AccountModel {
    pub id: u32,
    pub name: String
}
impl Model for AccountModel{}
impl AccountModel{
    pub fn show_name(&self)->String{
        return format!("{}-{}",self.name,self.id);
    }
}