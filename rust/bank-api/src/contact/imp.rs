use sqlx::{query_as, Pool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{contact::contact::{CAction, Contact, ContactFilter, ContactResponse}, error::BankError, Queriable};


impl Queriable<ContactFilter> for ContactResponse{

    fn generate_get_query<'a>(claims: &'a jwt_util::core::JwtClaims, params: &'a ContactFilter) -> QueryBuilder<'a, Postgres> {
        let mut qb = QueryBuilder::new("SELECT * FROM contacts c JOIN profiles p ON c.contact_id = p.user_id WHERE c.user_id = ");
        qb.push_bind(&claims.id);

        if let Some(contact_id) = &params.contact_id{
            qb.push(" AND c.contact_id = ");
            qb.push_bind(contact_id);
        }

        if let Some(contact_name) = &params.contact_name{
            qb.push("AND p.contact_name = ");
            qb.push_bind(contact_name);
        }

        if let Some(is_fav) = &params.is_fav{
            qb.push("AND c.is_fav = ");
            qb.push_bind(is_fav);
        }

        qb
    }
    
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &ContactFilter) -> Result<Self, BankError> where Self: Sized {
        Ok(ContactResponse { 
            contacts: ContactResponse::generate_get_query(claims, params).build_query_as().fetch_all(pool).await? 
        })
    }

    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &ContactFilter) -> Result<Self, BankError> where Self: Sized {
        if let (Some(action), Some(contact_id), user_id) = (&params.contact_action, &params.contact_id, &claims.id){
            match action {
                CAction::ADD => Self::add(contact_id, user_id, pool).await,
                CAction::REMOVE => Self::remove(contact_id, user_id, pool).await,
                CAction::FAV => Self::fav(contact_id, user_id, pool).await,
                CAction::UN_FAV => Self::un_fav(contact_id, user_id, pool).await,
            }
        }else{
            Err(BankError::InvalidParams)
        }
    }
}

impl ContactResponse{
    //SQL POST LOGIC
    pub(super) async fn add(contact_id: &Uuid, user_id: &Uuid, pool: &Pool<Postgres>) -> Result<Self, BankError>{
        const QUERY: &str = r#"
                INSERT INTO contacts (user_id, contact_id, is_fav)
                VALUES ($1, $2, FALSE) 
                ON CONFLICT (user_id, contact_id) 
                DO NOTHING 
                RETURNING user_id, contact_id, is_fav, contact_name
            "#;
        Self::internal_query(contact_id, user_id, pool, QUERY).await
    }
    pub(super) async fn remove(contact_id: &str, user_id: &str, pool: &Pool<Postgres>) -> Result<Self, BankError>{
        const QUERY: &str = r#"
                DELETE FROM contacts
                WHERE user_id = $1 AND contact_id = $2
                RETURNING user_id, contact_id, is_fav, contact_name
            "#;
        Self::internal_query(contact_id, user_id, pool, QUERY).await
    }
    pub(super) async fn fav(contact_id: &str, user_id: &str, pool: &Pool<Postgres>) -> Result<Self, BankError>{
        const QUERY: &str = r#"
                INSERT INTO contacts (user_id, contact_id, is_fav)
                VALUES ($1, $2, TRUE)
                ON CONFLICT (user_id, contact_id)
                DO UPDATE
                    SET is_fav = TRUE
                RETURNING user_id, contact_id, is_fav, contact_name
            "#;
        Self::internal_query(contact_id, user_id, pool, QUERY).await
        
    }
    pub(super) async fn un_fav(contact_id: &str, user_id: &str, pool: &Pool<Postgres>) -> Result<Self, BankError>{
        const QUERY: &str = r#"
                UPDATE contacts
                SET is_fav = FALSE
                WHERE user_id = $1 AND contact_id = $2
                RETURNING user_id, contact_id, is_fav, contact_name
            "#;
        Self::internal_query(contact_id, user_id, pool, QUERY).await
    }

    async fn internal_query(contact_id: &str, user_id: &str, pool: &Pool<Postgres>, query: &str)-> Result<Self, BankError>{
        Ok(ContactResponse{
            contacts: query_as::<_, Contact>(query).bind(user_id).bind(contact_id).fetch_all(pool).await?
        })
    }
}