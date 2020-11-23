use crate::diesel::Connection;
use crate::models::{user_assets::UserAsset, user_equipments::UserEquipment};
use crate::{ReqContext, ResponseResult};
use anyhow::{anyhow, Error};
use byteorder::{LittleEndian, ReadBytesExt};

pub async fn get_user_equipment_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let page = cursor.read_i16::<LittleEndian>()?;
    let uid = cursor.read_i64::<LittleEndian>()?;

    let limit = 20;

    let offset = limit * (page as i64);

    if uid <= 0 {
        return Err(anyhow!("invalid uid."));
    }

    let user_equipment_list =
        UserEquipment::get_equipment_list_data_collection(&conn, uid, limit, offset)?;

    req.get_bincode(200, "Success", user_equipment_list)
}

pub async fn user_buy_equipment(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let eid = cursor.read_i64::<LittleEndian>()?;
    let uid = cursor.read_i64::<LittleEndian>()?;
    let golds = cursor.read_i32::<LittleEndian>()?;

    if uid <= 0 || eid <= 0 {
        return Err(anyhow!("invalid param."));
    }

    conn.transaction::<(), Error, _>(|| {
        UserAsset::update_user_assets(&conn, uid, golds, 0)?;
        UserEquipment::user_buy_equipment(&conn, eid, uid)
    })?;

    req.get_bincode(200, "Success", "")
}
