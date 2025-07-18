use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct ContactResponse{
    pub(super) contacts: Vec<Contact> //list of contacts affect or retrieved
}

#[derive(Debug, Serialize)]
#[derive(sqlx::FromRow)]
#[serde(rename_all = "lowercase")]
pub struct Contact{
    user_id: Uuid,  //Contact Owner
    contact_name: String, //Display Name
    contact_id: Uuid, //Acount ID of Contact
    is_fav: Option<bool> //True if favorite, False if just saved, None if not saved
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct ContactFilter{
    pub contact_name: Option<String>, //filter by name of contact
    pub contact_id: Option<Uuid>, //filter by id of contact
    pub is_fav: Option<bool>, //filter by favorite
    pub contact_action: Option<CAction> //Action to perform
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CAction {
    ADD,
    REMOVE,
    FAV,
    UN_FAV,
}



