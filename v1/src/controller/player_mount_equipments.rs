use crate::models::player_mount_equipments::PlayerMountEquipment;
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use byteorder::{LittleEndian, ReadBytesExt};

pub async fn mount_user_player_equipment(req: ReqContext) -> ResponseResult {
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

    let equipment_id = cursor.read_i64::<LittleEndian>()?;
    if equipment_id <= 0 {
        return Err(anyhow!("invalid equipment_id."));
    }

    PlayerMountEquipment::mount_user_player_equipment(&conn, pid, uid, equipment_id)?;

    req.get_bincode(200, "Success", "")
}

pub async fn umount_user_player_equipment(req: ReqContext) -> ResponseResult {
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

    let equipment_id = cursor.read_i64::<LittleEndian>()?;
    if equipment_id <= 0 {
        return Err(anyhow!("invalid equipment_id."));
    }

    PlayerMountEquipment::umount_user_player_equipment(&conn, pid, uid, equipment_id)?;

    req.get_bincode(200, "Success", "")
}

pub async fn switch_user_player_equipment(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let id = cursor.read_i64::<LittleEndian>()?;
    if id <= 0 {
        return Err(anyhow!("invalid id."));
    }

    let uid = cursor.read_i64::<LittleEndian>()?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let equipment_id = cursor.read_i64::<LittleEndian>()?;
    if equipment_id <= 0 {
        return Err(anyhow!("invalid equipment_id."));
    }

    PlayerMountEquipment::switch_user_player_equipment(&conn, id, uid, equipment_id)?;

    req.get_bincode(200, "Success", "")
}
