use std::collections::HashMap;
use serde::Serialize;
use sqlx::{query, query_builder, Pool, Postgres, QueryBuilder, Row};

use crate::{ filter::ContactFilter, Queriable};

#[derive(Debug, Serialize)]
pub struct ContactResponse{
    contacts: Option<Vec<Contact>> //list of contacts affect or retrieved
}

#[derive(Debug, Serialize)]
#[derive(sqlx::FromRow)]
pub struct Contact{
    contact_name: String, //Display Name
    contact_id: String, //Acount ID
    is_fav: Option<bool> //True if favorite, False if just saved, None if not saved
}

impl Default for ContactResponse {
    fn default() -> Self {
        Self { contacts: None }
    }
}

impl Queriable<ContactFilter> for ContactResponse{
    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &ContactFilter) -> Result<Self, sqlx::Error> where Self: Sized {
        if let (Some(action), Some(contact_id), user_id) = (&params.action, &params.id, &claims.id){
            match action {
                crate::filter::CAction::ADD => add(contact_id, user_id, pool).await,
                crate::filter::CAction::REMOVE => remove(contact_id, user_id, pool).await,
                crate::filter::CAction::FAV => fav(contact_id, user_id, pool).await,
                crate::filter::CAction::UN_FAV => un_fav(contact_id, user_id, pool).await,
            }
        }else{
            Ok(ContactResponse::default())
        }
    }
    
    fn generate_get_query<'a>(claims: &'a jwt_util::core::JwtClaims, params: &'a ContactFilter) -> QueryBuilder<'a, Postgres> {
        let mut qb = QueryBuilder::new("SELECT * FROM contacts c JOIN profiles p ON c.contact_id = p.user_id WHERE c.user_id = ");
        qb.push_bind(&claims.id);

        if let Some(contact_id) = &params.id{
            qb.push(" AND c.contact_id = ");
            qb.push_bind(contact_id);
        }

        if let Some(contact_name) = &params.name{
            qb.push("AND p.name = ");
            qb.push_bind(contact_name);
        }

        if let Some(is_fav) = &params.fav{
            qb.push("AND c.is_fav = ");
            qb.push_bind(is_fav);
        }

        qb
    }
    
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &ContactFilter) -> Result<Self, sqlx::Error> where Self: Sized {
        Ok(ContactResponse { contacts: Some(ContactResponse::generate_get_query(claims, params).build_query_as().fetch_all(pool).await?) })
    }
}


//SQL POST LOGIC
async fn add(contact_id: &String, user_id: &String, pool: &Pool<Postgres>) -> Result<ContactResponse, sqlx::Error>{
    todo!()
}
async fn remove(contact_id: &String, user_id: &String, pool: &Pool<Postgres>) -> Result<ContactResponse, sqlx::Error>{
    todo!()
}
async fn fav(contact_id: &String, user_id: &String, pool: &Pool<Postgres>) -> Result<ContactResponse, sqlx::Error>{
    todo!()
}
async fn un_fav(contact_id: &String, user_id: &String, pool: &Pool<Postgres>) -> Result<ContactResponse, sqlx::Error>{
    todo!()
}
