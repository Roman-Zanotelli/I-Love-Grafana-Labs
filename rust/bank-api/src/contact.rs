use std::collections::HashMap;
use serde::Serialize;
use sqlx::{Pool, Postgres, Row};

use crate::{ ParsableRow, ParsableRows, Queriable};

#[derive(Debug, Serialize)]
pub struct ContactResponse{
    contacts: Option<Vec<Contact>> //list of contacts affect or retrieved
}

#[derive(Debug, Serialize)]
pub struct Contact{
    name: String, //Display Name
    id: String, //Acount ID
    saved: Option<bool> //True if favorite, False if just saved, None if not saved
}

impl ParsableRow for Contact{
    fn parse_row(row: &sqlx::postgres::PgRow) -> Self {
        Contact{
            name: row.get("name"),
            id: row.get("id"),
            saved: row.get("saved"),
        }
    }
}

impl ParsableRows for ContactResponse{
    fn parse_rows(rows: &Vec<sqlx::postgres::PgRow>) -> Self {
        todo!()
    }
}


impl Queriable for ContactResponse{
    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) -> Result<Self, sqlx::Error> where Self: Sized + ParsableRows {
        //TODO: Write SQL Statement
        let rows = sqlx::query("SQL TODO").fetch_all(pool).await?;
        Ok(Self::parse_rows(&rows))
    }
}

