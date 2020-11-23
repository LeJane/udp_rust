use crate::models::user_assets::UserAsset;
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use byteorder::{LittleEndian, ReadBytesExt};

pub async fn get_user_asset_info(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let uid = cursor.read_i64::<LittleEndian>()?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let user_asset_data = UserAsset::get_user_assets(&conn, uid)?;

    req.get_bincode(200, "Success", user_asset_data)
}

pub async fn update_user_asset_info(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let uid = cursor.read_i64::<LittleEndian>()?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }
    let golds = cursor.read_i32::<LittleEndian>()?;

    let diamonds = cursor.read_i32::<LittleEndian>()?;

    UserAsset::update_user_assets(&conn, uid, golds, diamonds)?;

    req.get_bincode(200, "Success", "")
}
