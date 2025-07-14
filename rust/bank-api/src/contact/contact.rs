use serde::Serialize;
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::{ filter::ContactFilter, Queriable};

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct ContactResponse{
    pub(super) contacts: Option<Vec<Contact>> //list of contacts affect or retrieved
}

#[derive(Debug, Serialize)]
#[derive(sqlx::FromRow)]
#[serde(rename_all = "lowercase")]
pub struct Contact{
    contact_name: String, //Display Name
    contact_id: String, //Acount ID
    is_fav: Option<bool> //True if favorite, False if just saved, None if not saved
}




