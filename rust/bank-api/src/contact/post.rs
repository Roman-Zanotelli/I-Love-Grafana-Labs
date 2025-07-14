use sqlx::{query_as, Pool, Postgres};

use crate::contact::contact::{Contact, ContactResponse};

//SQL POST LOGIC
pub(super) async fn add(contact_id: &str, user_id: &str, pool: &Pool<Postgres>) -> Result<ContactResponse, sqlx::Error>{
    Ok(ContactResponse{
        contacts: Some(query_as::<_, Contact>(r#"
            INSERT INTO contacts (user_id, contact_id, is_fav)
            VALUES ($1, $2, FALSE) 
            ON CONFLICT (user_id, contact_id) 
            DO NOTHING 
            RETURNING user_id, contact_id, is_fav, contact_name
        "#).bind(user_id).bind(contact_id).fetch_all(pool).await?),
    })
}
pub(super) async fn remove(contact_id: &str, user_id: &str, pool: &Pool<Postgres>) -> Result<ContactResponse, sqlx::Error>{
    Ok(ContactResponse{
        contacts: Some(query_as::<_, Contact>(r#"
            DELETE FROM contacts
            WHERE user_id = $1 AND contact_id = $2
            RETURNING user_id, contact_id, is_fav, contact_name
        "#).bind(user_id).bind(contact_id).fetch_all(pool).await?),
    })
}
pub(super) async fn fav(contact_id: &str, user_id: &str, pool: &Pool<Postgres>) -> Result<ContactResponse, sqlx::Error>{
    Ok(ContactResponse{
        contacts: Some(query_as::<_, Contact>(r#"
            INSERT INTO contacts (user_id, contact_id, is_fav)
            VALUES ($1, $2, TRUE)
            ON CONFLICT (user_id, contact_id)
            DO UPDATE
                SET is_fav = TRUE
            RETURNING user_id, contact_id, is_fav, contact_name
        "#).bind(user_id).bind(contact_id).fetch_all(pool).await?),
    })
}
pub(super) async fn un_fav(contact_id: &str, user_id: &str, pool: &Pool<Postgres>) -> Result<ContactResponse, sqlx::Error>{
    Ok(ContactResponse{
        contacts: Some(query_as::<_, Contact>(r#"
            UPDATE contacts
            SET is_fav = FALSE
            WHERE user_id = $1 AND contact_id = $2
            RETURNING user_id, contact_id, is_fav, contact_name
        "#).bind(user_id).bind(contact_id).fetch_all(pool).await?),
    })
}

