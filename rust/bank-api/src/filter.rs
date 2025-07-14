use serde::Deserialize;

use crate::transaction::{action::TAction, status::TStatus};




#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct ContactFilter{
    pub contact_name: Option<String>, //filter by name of contact
    pub contact_id: Option<String>, //filter by id of contact
    pub is_fav: Option<bool>, //filter by favorite
    pub contact_action: Option<CAction> //action to perform
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CAction {
    ADD,
    REMOVE,
    FAV,
    UN_FAV,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct TransactionFilter{
    pub transaction_id: Option<String>, //filter by transaction id
    pub contact_id: Option<String>, //filter by contact id
    pub status: Option<TStatus>, // filter by pending status
    pub action: Option<TAction>, //action to preform (POST)
    pub less_than: Option<i32>, //less than amount
    pub more_than: Option<i32> //more than amount
}

