use crate::models::enemys::Enemy;
use crate::{ReqContext, ResponseResult};

pub async fn get_enemy_list_data(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn()?;

    let enemys_data = Enemy::get_enemy_data_list(&conn)?;

    req.get_bincode(200, "Success", enemys_data)
}
