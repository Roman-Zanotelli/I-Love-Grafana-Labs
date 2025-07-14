use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct ContactFilter{
    pub name: Option<String>, //filter by name of contact
    pub id: Option<String>, //filter by id of contact
    pub fav: Option<bool>, //filter by favorite
    pub action: Option<CAction> //action to perform
}

#[derive(Deserialize, Debug)]
pub enum CAction {
    ADD,
    REMOVE,
    FAV,
    UN_FAV,
}


#[derive(Deserialize, Debug)]
pub struct TransactionFilter{
    pub transaction: Option<String>, //filter by transaction id
    pub contact: Option<String>, //filter by contact id
    pub pending: Option<bool>, // filter by pending status
    pub action: Option<crate::transaction::TAction> //action to preform (POST)
}

