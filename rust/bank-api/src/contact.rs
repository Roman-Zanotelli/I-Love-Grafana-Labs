use std::collections::HashMap;
use serde::Serialize;
use sqlx::{query, Pool, Postgres, QueryBuilder, Row};

use crate::{ filter::ContactFilter, ParsableRow, ParsableRows, Queriable};

#[derive(Debug, Serialize)]
pub struct ContactResponse{
    contacts: Option<Vec<Contact>> //list of contacts affect or retrieved
}

#[derive(Debug, Serialize)]
pub struct Contact{
    contact_name: String, //Display Name
    contact_id: String, //Acount ID
    is_fav: Option<bool> //True if favorite, False if just saved, None if not saved
}

impl ParsableRow for Contact{
    fn parse_row(row: &sqlx::postgres::PgRow) -> Self {
        Contact{
            contact_name: row.get("name"),
            contact_id: row.get("contact_id"),
            is_fav: row.get("is_fav"),
        }
    }
}

impl ParsableRows for ContactResponse{
    fn parse_rows(rows: &Vec<sqlx::postgres::PgRow>) -> Self {
        todo!()
    }
}


impl Queriable<ContactFilter> for ContactResponse{
    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &ContactFilter) -> Result<Self, sqlx::Error> where Self: Sized + ParsableRows {
        //TODO: Write SQL Statement
        // let rows = sqlx::query("SQL TODO").fetch_all(pool).await?;
        // Ok(Self::parse_rows(&rows))
        todo!()
    }
    
    fn generate_get_query<'a>(claims: &'a jwt_util::core::JwtClaims, params: &'a ContactFilter) -> QueryBuilder<'a, Postgres> {
        let mut qb = QueryBuilder::new("SELECT * FROM contacts c JOIN profiles p ON c.contact_id = p.user_id WHERE c.user_id = ");
        qb.push_bind(&claims.id);

        if let Some(contact_id) = &params.contact_id{
            qb.push(" AND c.contact_id = ");
            qb.push_bind(contact_id);
        }

        if let Some(contact_name) = &params.contact_name{
            qb.push("AND p.name = ");
            qb.push_bind(contact_name);
        }

        if let Some(is_fav) = &params.is_fav{
            qb.push("AND c.is_fav = ");
            qb.push_bind(is_fav);
        }

        qb
    }
}

