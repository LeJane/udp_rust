use crate::models::equipments::Equipment;
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use byteorder::{LittleEndian, ReadBytesExt};

pub async fn get_equipment_list_by_kid(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let mut cursor = std::io::Cursor::new(&req.body);
    let page = cursor.read_i16::<LittleEndian>()?;
    let kid = cursor.read_i64::<LittleEndian>()?;

    let limit = 20;

    let offset = limit * (page as i64);

    if kid <= 0 {
        return Err(anyhow!("invalid equipment kind id."));
    }

    let equipment_kind_list = Equipment::get_shop_equipment_data_by_kid(&conn, kid, limit, offset)?;

    req.get_bincode(200, "Success", equipment_kind_list)
}
