use crate::schema::server_lists;
use crate::BinaryEncode;
use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(slid)]
pub struct ServerList {
    pub slid: i64,
    pub name: String,
    pub country_code: String,
    pub area: String,
    pub ip: String,
    pub port: i16,
    pub server_type: i16, //1:chat,2:api
    pub state: i16,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct FrontDisplayServer {
    pub ip: String,
    pub port: i16,
    pub server_type: i16, //1:chat,2:api
}

#[derive(Insertable, Debug, Default)]
#[table_name = "server_lists"]
pub struct NewServerList<'a> {
    pub slid: i64,
    pub name: &'a str,
    pub country_code: &'a str,
    pub area: &'a str,
    pub ip: &'a str,
    pub port: i16,
    pub server_type: i16, //1:chat,2:api
    pub state: i16,
}

impl ServerList {
    pub fn get_server_list_ip_port(conn: &PgConnection) -> QueryResult<Vec<FrontDisplayServer>> {
        server_lists::table
            .filter(server_lists::state.eq(0))
            .select((
                server_lists::ip,
                server_lists::port,
                server_lists::server_type,
            ))
            .get_results::<FrontDisplayServer>(conn)
    }
}

impl BinaryEncode for FrontDisplayServer {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        let ip = self.ip.as_bytes();
        encoded.write_i32::<LittleEndian>(ip.len() as i32)?;
        encoded.extend_from_slice(ip);
        encoded.write_i16::<LittleEndian>(self.port)?;
        encoded.write_i16::<LittleEndian>(self.server_type)?;

        //set item length
        encoded.encode()
    }
}
