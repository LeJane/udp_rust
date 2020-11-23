use crate::models::user_players::UserPlayer;
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use byteorder::{LittleEndian, ReadBytesExt};

pub async fn get_user_default_player_data(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let uid = cursor.read_i64::<LittleEndian>()?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let user_player_data = UserPlayer::get_user_default_player_data(&conn, uid as i64)?;

    req.get_bincode(200, "Success", user_player_data)
}

pub async fn get_player_collection_data(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let uid = cursor.read_i64::<LittleEndian>()?;
    let pid = cursor.read_i64::<LittleEndian>()?;

    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if pid <= 0 {
        return Err(anyhow!("invalid player id."));
    }

    let enemys_data = UserPlayer::get_player_data_collection_by_pid(&conn, uid as i64, pid as i64)?;

    req.get_bincode(200, "Success", enemys_data)
}
