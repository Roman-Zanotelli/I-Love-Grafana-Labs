use sqlx::{Pool, Postgres, QueryBuilder};

use crate::{contact::{contact::ContactResponse, post::*}, filter::ContactFilter, Queriable};

impl Default for ContactResponse {
    fn default() -> Self {
        Self { contacts: None }
    }
}

impl Queriable<ContactFilter> for ContactResponse{
    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &ContactFilter) -> Result<Self, sqlx::Error> where Self: Sized {
        if let (Some(action), Some(contact_id), user_id) = (&params.contact_action, &params.contact_id, &claims.id){
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
    
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &ContactFilter) -> Result<Self, sqlx::Error> where Self: Sized {
        Ok(ContactResponse { contacts: Some(ContactResponse::generate_get_query(claims, params).build_query_as().fetch_all(pool).await?) })
    }
}