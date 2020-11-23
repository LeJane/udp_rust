use crate::schema::chat_groups;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable, Associations)]
#[primary_key(gid)]
pub struct ChatGroup {
    pub gid: i64,
    pub group_name: String,
    pub uuid: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "chat_groups"]
pub struct NewChatGroup<'a> {
    pub gid: i64,
    pub group_name: &'a str,
    pub uuid: i64,
}
