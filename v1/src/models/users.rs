use crate::get_guid_value;
use crate::models::servers as servers_models;
use crate::schema::{servers, user_assets, users};
use crate::BinaryEncode;
use anyhow::{bail, Context, Result};
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Identifiable, Queryable, Associations)]
#[primary_key(uuid)]
pub struct User {
    pub uuid: i64,
    pub uid: i32,
    pub name: String,
    pub avatar: String,
    pub login_days: i32,
    pub server_id: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
    pub action_force: i32,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub uuid: i64,
    pub uid: i32,
    pub name: &'a str,
    pub avatar: &'a str,
    pub login_days: i32,
    pub server_id: i32,
    pub action_force: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct FrontDisplayUser {
    pub uuid: i64,
    pub uid: i32,
    pub name: String,
    pub avatar: String,
    pub server_id: i32,
    pub action_force: i32,
    pub golds: i32,
    pub diamonds: i32,
}

impl User {
    pub fn create_user(conn: &PgConnection, uid: i32) -> Result<(Self, servers_models::Server)> {
        let uuid = get_guid_value();

        let name = format!("Governor{}", uid);
        let mut rng = rand::thread_rng();
        let rand_value: u64 = rng.gen_range(1, 3);
        let avatar = format!("game://{}", rand_value);

        let server = servers_models::Server::get_server_socket_latest(conn)
            .with_context(|| format!("Failed to get server socket"))?;

        let user = NewUser {
            uuid: uuid as i64,
            uid: uid as i32,
            name: &name,
            avatar: &avatar,
            login_days: 1,
            server_id: server.server_number,
            action_force: 1000,
        };

        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(conn)?;
        Ok((user, server))
    }

    pub fn get_user_server_socket(conn: &PgConnection, uuid: i64) -> QueryResult<(String, i16)> {
        users::table
            .filter(users::uuid.eq(uuid))
            .inner_join(servers::table.on(users::server_id.eq(servers::server_number)))
            .select((servers::ip, servers::ports))
            .first(conn)
    }

    pub fn get_user_base_info(conn: &PgConnection, uuid: i64) -> QueryResult<FrontDisplayUser> {
        users::table
            .filter(users::uuid.eq(uuid))
            .inner_join(user_assets::table.on(users::uuid.eq(user_assets::uid)))
            .select((
                users::uuid,
                users::uid,
                users::name,
                users::avatar,
                users::server_id,
                users::action_force,
                user_assets::golds,
                user_assets::diamonds,
            ))
            .first(conn)
    }

    pub fn update_user_action_force(
        conn: &PgConnection,
        uid: i64,
        action_force: i32,
    ) -> Result<()> {
        let old_action_force = users::table
            .filter(users::uuid.eq(uid))
            .select(users::action_force)
            .first::<i32>(conn)?;

        if action_force < 0 && (old_action_force + action_force) < 0 {
            bail!("user action force not enough.");
        }

        diesel::update(users::table)
            .set(users::action_force.eq(users::action_force + action_force))
            .filter(users::uuid.eq(uid))
            .execute(conn)?;

        Ok(())
    }

    pub fn update_user_name(conn: &PgConnection, uid: i64, name: String) -> Result<()> {
        diesel::update(users::table)
            .set(users::name.eq(name))
            .filter(users::uuid.eq(uid))
            .execute(conn)?;

        Ok(())
    }
}

impl BinaryEncode for FrontDisplayUser {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        encoded.write_i64::<LittleEndian>(self.uuid)?;
        encoded.write_i32::<LittleEndian>(self.uid)?;
        let name = self.name.as_bytes();
        encoded.write_i32::<LittleEndian>(name.len() as i32)?;
        encoded.extend_from_slice(name);

        let avatar = self.avatar.as_bytes();
        encoded.write_i32::<LittleEndian>(avatar.len() as i32)?;
        encoded.extend_from_slice(avatar);
        encoded.write_i32::<LittleEndian>(self.server_id)?;
        encoded.write_i32::<LittleEndian>(self.action_force)?;
        encoded.write_i32::<LittleEndian>(self.golds)?;
        encoded.write_i32::<LittleEndian>(self.diamonds)?;

        //set item length
        encoded.encode()
    }
}
