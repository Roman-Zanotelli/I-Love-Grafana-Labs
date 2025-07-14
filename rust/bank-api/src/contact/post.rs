use sqlx::{Pool, Postgres};

use crate::contact::contact::ContactResponse;

//SQL POST LOGIC
pub(super) async fn add(contact_id: &String, user_id: &String, pool: &Pool<Postgres>) -> Result<ContactResponse, sqlx::Error>{
    todo!()
}
pub(super) async fn remove(contact_id: &String, user_id: &String, pool: &Pool<Postgres>) -> Result<ContactResponse, sqlx::Error>{
    todo!()
}
pub(super) async fn fav(contact_id: &String, user_id: &String, pool: &Pool<Postgres>) -> Result<ContactResponse, sqlx::Error>{
    todo!()
}
pub(super) async fn un_fav(contact_id: &String, user_id: &String, pool: &Pool<Postgres>) -> Result<ContactResponse, sqlx::Error>{
    todo!()
}
