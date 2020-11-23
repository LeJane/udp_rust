use crate::models::user_player_tracks::UserPlayerTrack;
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use byteorder::{LittleEndian, ReadBytesExt};

pub async fn insert_or_update_player_track(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let pid = cursor.read_i64::<LittleEndian>()?;
    if pid <= 0 {
        return Err(anyhow!("invalid pid."));
    }

    let uid = cursor.read_i64::<LittleEndian>()?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let rotaion_x = cursor.read_f32::<LittleEndian>()?;
    let rotaion_y = cursor.read_f32::<LittleEndian>()?;
    let rotaion_z = cursor.read_f32::<LittleEndian>()?;
    let location_x = cursor.read_f32::<LittleEndian>()?;
    let location_y = cursor.read_f32::<LittleEndian>()?;
    let location_z = cursor.read_f32::<LittleEndian>()?;

    UserPlayerTrack::insert_or_update_player_track(
        &conn, pid, uid, rotaion_x, rotaion_y, rotaion_z, location_x, location_y, location_z,
    )?;

    req.get_bincode(200, "Success", "")
}

pub async fn get_player_track(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let pid = cursor.read_i64::<LittleEndian>()?;
    if pid <= 0 {
        return Err(anyhow!("invalid player id."));
    }
    let uid = cursor.read_i64::<LittleEndian>()?;

    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let track = UserPlayerTrack::get_player_track(&conn, pid, uid)?;

    req.get_bincode(200, "Success", track)
}
