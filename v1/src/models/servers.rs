use crate::schema::servers;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(sid)]
pub struct Server {
    pub sid: i64,
    pub server_number: i32,
    pub name: String,
    pub ip: String,
    pub ports: i16,
    pub person_count: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Insertable, Debug, Default)]
#[table_name = "servers"]
pub struct NewServer<'a> {
    pub sid: i64,
    pub server_number: i32,
    pub name: &'a str,
    pub ip: &'a str,
    pub ports: i16,
    pub person_count: i32,
}

impl Server {
    pub fn get_server_socket_latest(conn: &PgConnection) -> QueryResult<Server> {
        servers::table
            .order_by(servers::person_count.desc())
            .first(conn)
    }

    pub fn update_server_person_count(
        conn: &PgConnection,
        sid: i64,
        value: i32,
    ) -> QueryResult<()> {
        diesel::update(servers::table.filter(servers::sid.eq(sid)))
            .set(servers::person_count.eq(value))
            .execute(conn)?;

        Ok(())
    }
}
