use crate::models::equipment_kinds::EquipmentKind;
use crate::{ReqContext, ResponseResult};

pub async fn get_equipment_kind_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let equipment_kind_list = EquipmentKind::get_equipment_kind_list(&conn)?;

    req.get_bincode(200, "Success", equipment_kind_list)
}
