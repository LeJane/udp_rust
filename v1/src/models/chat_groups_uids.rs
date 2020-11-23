use crate::schema::{chat_groups_uids, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Identifiable, Queryable, Associations)]
#[primary_key(guid)]
pub struct ChatGroupsUid {
    pub guid: i64,
    pub gid: i64,
    pub uuid: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "chat_groups_uids"]
pub struct NewChatGroupUid {
    pub gid: i64,
    pub uuid: i64,
}

impl ChatGroupsUid {
    pub fn get_groups_users_by_gid(
        conn: &PgConnection,
        gid: i64,
    ) -> QueryResult<Vec<(i64, i64, i32)>> {
        chat_groups_uids::table
            .filter(chat_groups_uids::gid.eq(gid))
            .inner_join(users::table.on(users::uuid.eq(chat_groups_uids::uuid)))
            .select((
                chat_groups_uids::gid,
                chat_groups_uids::uuid,
                users::server_id,
            ))
            .load(conn)
    }
}
