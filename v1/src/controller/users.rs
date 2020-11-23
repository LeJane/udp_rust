use crate::models::users::{FrontDisplayUser, User};
use crate::models::{
    servers::Server, user_assets::UserAsset, user_equipments::UserEquipment,
    user_players::UserPlayer,
};
use crate::GLOBAL_USERS_COUNTER_REDIS_KEY;
use crate::{ReqContext, ResponseResult};
use anyhow::{anyhow, Error};
use byteorder::{LittleEndian, ReadBytesExt};
use diesel::prelude::*;
use redis::Commands;
use tracing::error;

pub async fn create_user(req: ReqContext) -> ResponseResult {
    let state = req.state.lock().await;
    std::mem::forget(&state);
    let mut redis_conn = req.redis_conn()?;
    let conn = req.db_conn()?;

    let old_uid_counter: u64 = match redis_conn.get(GLOBAL_USERS_COUNTER_REDIS_KEY) {
        Ok(v) => v,
        Err(e) => {
            error!("failed to redis get global user counter:{}", e);
            0
        }
    };

    let new_uid_counter = old_uid_counter + 1;

    let user = match conn.transaction::<User, Error, _>(|| {
        let _uid_counter = redis_conn.incr::<&str, u64, u32>(GLOBAL_USERS_COUNTER_REDIS_KEY, 1)?;
        let (user, server) = User::create_user(&conn, 10000000 + new_uid_counter as i32)?;
        Server::update_server_person_count(&conn, server.sid, server.person_count + 1)?;
        UserAsset::create_user_assets(&conn, user.uuid)?;

        //create user default player
        UserPlayer::create_user_default_player(&conn, user.uuid)?;

        //create user default equipment
        UserEquipment::create_user_default_equipments(&conn, user.uuid)?;
        Ok(user)
    }) {
        Ok(v) => v,
        Err(e) => {
            let _uid_counter =
                redis_conn.incr::<&str, i64, u32>(GLOBAL_USERS_COUNTER_REDIS_KEY, -1)?;
            return Err(anyhow!("failed create user info:{}", e));
        }
    };

    let return_user_iunfo = FrontDisplayUser {
        uuid: user.uuid,
        uid: user.uid,
        name: user.name,
        avatar: user.avatar,
        server_id: user.server_id,
        action_force: user.action_force,
        golds: 0,
        diamonds: 0,
    };

    std::mem::drop(state);

    req.get_bincode(200, "Success", return_user_iunfo)
}

pub async fn get_user_base_info(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let uid = cursor.read_i64::<LittleEndian>()?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let user_base_data = User::get_user_base_info(&conn, uid)?;

    req.get_bincode(200, "Success", user_base_data)
}

pub async fn update_user_action_force(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let uid = cursor.read_i64::<LittleEndian>()?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }
    let action_force = cursor.read_i32::<LittleEndian>()?;

    User::update_user_action_force(&conn, uid, action_force)?;

    req.get_bincode(200, "Success", "")
}

pub async fn update_user_name(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let uid = cursor.read_i64::<LittleEndian>()?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }
    let name_length = cursor.read_i32::<LittleEndian>()?;
    if name_length <= 0 {
        return Err(anyhow!("invalid name length."));
    }
    let position = cursor.position() as usize;
    let name = &req.body[position..(position + name_length as usize)];
    let name = std::str::from_utf8(name)?;
    User::update_user_name(&conn, uid, name.into())?;

    req.get_bincode(200, "Success", "")
}
