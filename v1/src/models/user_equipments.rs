use super::equipments::{Equipment, EquipmentData};
use crate::get_guid_value;
use crate::schema::{equipments, user_equipments};
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Identifiable, Queryable)]
pub struct UserEquipment {
    pub id: i64,
    pub eid: i64,
    pub uid: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "user_equipments"]
pub struct NewUserEquipment {
    pub id: i64,
    pub eid: i64,
    pub uid: i64,
}

impl UserEquipment {
    pub fn get_equipment_list_data_collection(
        conn: &PgConnection,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> QueryResult<Vec<EquipmentData>> {
        let equipments = user_equipments::table
            .filter(user_equipments::uid.eq(uid))
            .inner_join(equipments::table.on(equipments::eid.eq(user_equipments::eid)))
            .select(equipments::all_columns)
            .limit(limit)
            .offset(offset)
            .load::<Equipment>(conn)?;

        Equipment::get_equipment_data_collection(conn, equipments)
    }

    pub fn create_user_default_equipments(conn: &PgConnection, uid: i64) -> QueryResult<()> {
        //get default equipment id
        let eids = Equipment::get_default_equipment_id_list(conn)?;

        let mut user_equipments = Vec::new();
        eids.into_iter().for_each(|eid| {
            let user_equipment = NewUserEquipment {
                id: get_guid_value() as i64,
                eid: eid,
                uid: uid,
            };
            user_equipments.push(user_equipment);
        });

        let _usize = diesel::insert_into(user_equipments::table)
            .values(user_equipments)
            .execute(conn)?;

        Ok(())
    }

    pub fn user_buy_equipment(conn: &PgConnection, eid: i64, uid: i64) -> Result<()> {
        let id: i64 = match user_equipments::table
            .filter(user_equipments::eid.eq(eid))
            .filter(user_equipments::uid.eq(uid))
            .select(user_equipments::id)
            .first(conn)
        {
            Ok(v) => v,
            Err(_e) => 0,
        };

        if id > 0 {
            return Err(anyhow!("equipment has ."));
        }

        let user_equipment = NewUserEquipment {
            id: get_guid_value() as i64,
            eid: eid,
            uid: uid,
        };

        diesel::insert_into(user_equipments::table)
            .values(user_equipment)
            .execute(conn)?;

        Ok(())
    }
}
