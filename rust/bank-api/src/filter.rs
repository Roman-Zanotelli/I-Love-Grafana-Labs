use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct ContactFilter{
    pub contact_name: Option<String>,
    pub contact_id: Option<String>,
    pub is_fav: Option<bool>
}

#[derive(Deserialize, Debug)]
pub struct TransactionFilter{
    pub transaction_id: Option<String>,
    pub contact_id: Option<String>,
    pub include_pending: Option<bool>
}
