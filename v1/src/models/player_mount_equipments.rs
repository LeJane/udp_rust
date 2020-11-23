use super::equipments::Equipment;
use crate::get_guid_value;
use crate::schema::player_mount_equipments;
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Identifiable, Queryable)]
pub struct PlayerMountEquipment {
    pub id: i64,
    pub pid: i64,
    pub uid: i64,
    pub equipment_id: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "player_mount_equipments"]
pub struct NewPlayerMountEquipment {
    pub id: i64,
    pub pid: i64,
    pub uid: i64,
    pub equipment_id: i64,
}

impl PlayerMountEquipment {
    pub fn get_player_mount_equipment_list(
        conn: &PgConnection,
        pid: i64,
        uid: i64,
    ) -> QueryResult<Vec<Equipment>> {
        let equipment_ids = player_mount_equipments::table
            .filter(player_mount_equipments::pid.eq(pid))
            .filter(player_mount_equipments::uid.eq(uid))
            .select(player_mount_equipments::equipment_id)
            .get_results::<i64>(conn)?;

        let mut datas = Vec::new();

        for equipment_id in equipment_ids {
            let equipment = Equipment::get_equipment_data_by_id(conn, equipment_id)?;
            datas.push(equipment);
        }

        Ok(datas)
    }

    pub fn mount_user_player_equipment(
        conn: &PgConnection,
        pid_value: i64,
        uid_value: i64,
        equipment_id_value: i64,
    ) -> Result<()> {
        use crate::schema::player_mount_equipments::dsl::*;

        let equipment_used_id: i64 = match player_mount_equipments
            .filter(uid.eq(uid_value))
            .filter(equipment_id.eq(equipment_id_value))
            .select(id)
            .get_result(conn)
        {
            Ok(v) => v,
            Err(_e) => 0,
        };

        if equipment_used_id > 0 {
            return Err(anyhow!("user equipment has mounted."));
        }

        let data = NewPlayerMountEquipment {
            id: get_guid_value() as i64,
            pid: pid_value,
            uid: uid_value,
            equipment_id: equipment_id_value,
        };

        diesel::insert_into(player_mount_equipments)
            .values(data)
            .execute(conn)?;

        Ok(())
    }

    pub fn umount_user_player_equipment(
        conn: &PgConnection,
        pid_value: i64,
        uid_value: i64,
        equipment_id_value: i64,
    ) -> Result<()> {
        diesel::delete(player_mount_equipments::table)
            .filter(player_mount_equipments::pid.eq(pid_value))
            .filter(player_mount_equipments::uid.eq(uid_value))
            .filter(player_mount_equipments::equipment_id.eq(equipment_id_value))
            .execute(conn)?;

        Ok(())
    }

    pub fn switch_user_player_equipment(
        conn: &PgConnection,
        id_value: i64,
        uid_value: i64,
        equipment_id_value: i64,
    ) -> Result<()> {
        use crate::schema::player_mount_equipments::dsl::*;

        let equipment_used_id: i64 = match player_mount_equipments
            .filter(uid.eq(uid_value))
            .filter(equipment_id.eq(equipment_id_value))
            .select(id)
            .get_result(conn)
        {
            Ok(v) => v,
            Err(_e) => 0,
        };

        if equipment_used_id > 0 {
            return Err(anyhow!("user equipment has mounted."));
        }

        diesel::update(player_mount_equipments)
            .set(equipment_id.eq(equipment_id_value))
            .filter(id.eq(id_value))
            .execute(conn)?;

        Ok(())
    }
}
