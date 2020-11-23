use crate::schema::link_accounts;
use chrono::NaiveDateTime;

#[derive(Debug, Identifiable, Queryable, Clone)]
#[primary_key(lid)]
pub struct LinkAccount {
    pub lid: u64,
    pub uuid: u64,
    pub account_type: u16,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Insertable, Debug, Default)]
#[table_name = "link_accounts"]
pub struct NewLinkAccount {
    pub lid: i64,
    pub uuid: i64,
    pub account_type: i16,
}
