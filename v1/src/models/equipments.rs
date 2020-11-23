use super::{
    gem_relateds::{FrontDisplayGem, GemRelated},
    skill_fight_relateds::{FrontDisplaySkill, SkillFightRelated},
};
use crate::schema::{equipments, user_equipments};
use crate::BinaryEncode;
use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(eid)]
pub struct Equipment {
    pub eid: i64,
    pub kid: i64,
    pub name: String,
    pub thumbnail: String,
    pub price: i32,
    pub hp: i32,
    pub multiplier: f32,
    pub kind: i16,       //1:weapons,2:armors,3:shields
    pub is_default: i16, //1:No,2:Yes
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EquipmentData {
    pub eid: i64,
    pub name: String,
    pub thumbnail: String,
    pub price: i32,
    pub hp: i32,
    pub multiplier: f32,
    pub kind: i16,
    pub gems: Vec<FrontDisplayGem>,
    pub skills: Vec<FrontDisplaySkill>,
}

// ALTER TABLE equipments ALTER COLUMN multiplier TYPE REAL;
#[derive(Debug, Default, Insertable)]
#[table_name = "equipments"]
pub struct NewEquipment<'a> {
    pub eid: i64,
    pub kid: i64,
    pub name: &'a str,
    pub thumbnail: &'a str,
    pub price: i32,
    pub hp: i32,
    pub multiplier: f32,
    pub kind: i16,
    pub is_default: i16, //1:No,2:Yes
}

impl Equipment {
    pub fn get_equipment_list(conn: &PgConnection) -> QueryResult<Vec<Equipment>> {
        equipments::table.get_results(conn)
    }

    pub fn get_equipment_data_by_id(conn: &PgConnection, eid: i64) -> QueryResult<Equipment> {
        equipments::table.find(eid).first(conn)
    }

    pub fn get_shop_equipment_data_by_kid(
        conn: &PgConnection,
        kid: i64,
        limit: i64,
        offset: i64,
    ) -> QueryResult<Vec<EquipmentData>> {
        let equipments = equipments::table
            .filter(equipments::kid.eq(kid))
            .limit(limit)
            .offset(offset)
            .get_results::<Equipment>(conn)?;

        let mut filter_equipments = Vec::new();
        for equipment in equipments.into_iter() {
            let id: i64 = match user_equipments::table
                .filter(user_equipments::eid.eq(equipment.eid))
                .select(user_equipments::id)
                .first(conn)
            {
                Ok(v) => v,
                Err(_e) => 0,
            };

            if id > 0 {
                continue;
            }

            filter_equipments.push(equipment);
        }

        Equipment::get_equipment_data_collection(conn, filter_equipments)
    }

    pub fn get_equipment_data_collection(
        conn: &PgConnection,
        equipments: Vec<Equipment>,
    ) -> QueryResult<Vec<EquipmentData>> {
        let mut terms = Vec::new();

        for equipment in equipments.iter() {
            let skills = SkillFightRelated::get_front_display_skill_list(conn, equipment.eid, 2)?;
            let gems = GemRelated::get_front_display_gem_related_list(conn, equipment.eid, 2)?;

            let term = EquipmentData {
                eid: equipment.eid,
                name: equipment.name.clone(),
                thumbnail: equipment.thumbnail.clone(),
                price: equipment.price,
                hp: equipment.hp,
                multiplier: equipment.multiplier,
                kind: equipment.kind,
                gems: gems,
                skills: skills,
            };

            terms.push(term);
        }

        Ok(terms)
    }

    pub fn get_default_equipment_id_list(conn: &PgConnection) -> QueryResult<Vec<i64>> {
        equipments::table
            .filter(equipments::is_default.eq(2))
            .select(equipments::eid)
            .get_results(conn)
    }
}

impl BinaryEncode for EquipmentData {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        encoded.write_i64::<LittleEndian>(self.eid)?;
        let name = self.name.as_bytes();
        encoded.write_i32::<LittleEndian>(name.len() as i32)?;
        encoded.extend_from_slice(name);
        let thumbnail = self.thumbnail.as_bytes();
        encoded.write_i32::<LittleEndian>(thumbnail.len() as i32)?;
        encoded.extend_from_slice(thumbnail);
        encoded.write_i32::<LittleEndian>(self.price)?;
        encoded.write_i32::<LittleEndian>(self.hp)?;
        encoded.write_f32::<LittleEndian>(self.multiplier)?;
        encoded.write_i16::<LittleEndian>(self.kind)?;
        //gems
        let gems = self.gems.encode()?;
        encoded.extend(gems);
        //skills
        let skills = self.skills.encode()?;
        encoded.extend(skills);

        //set item length
        encoded.encode()
    }
}
