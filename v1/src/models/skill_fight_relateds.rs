use crate::schema::{skill_fight_relateds, skills};
use crate::BinaryEncode;
use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Queryable)]
pub struct SkillFightRelated {
    pub id: i64,
    pub obj_id: i64,
    pub skill_id: i64,
    pub cool_down: i32,
    pub attack_power: i32,
    pub mana_power: i32,
    pub probability: i16,
    pub level: i16,
    pub level_experience: i32,
    pub obj_type: i16,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct FrontDisplaySkill {
    pub thumbnail: String,
    pub skill_name: String,
    pub skill_description: String,
    pub cool_down: i32,
    pub attack_power: i32,
    pub mana_power: i32,
    pub probability: i16,
    pub level: i16,
    pub level_experience: i32,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "skill_fight_relateds"]
pub struct NewSkillFigthRelated {
    pub id: i64,
    pub obj_id: i64,
    pub skill_id: i64,
    pub cool_down: i32,
    pub attack_power: i32,
    pub mana_power: i32,
    pub probability: i16,
    pub level: i16,
    pub level_experience: i32,
    pub obj_type: i16,
}

impl SkillFightRelated {
    pub fn get_front_display_skill_list(
        conn: &PgConnection,
        obj_id: i64,
        obj_type: i16,
    ) -> QueryResult<Vec<FrontDisplaySkill>> {
        skill_fight_relateds::table
            .filter(skill_fight_relateds::obj_id.eq(obj_id))
            .filter(skill_fight_relateds::obj_type.eq(obj_type))
            .inner_join(skills::table.on(skills::id.eq(skill_fight_relateds::skill_id)))
            .select((
                skills::thumbnail,
                skills::skill_name,
                skills::skill_description,
                skill_fight_relateds::cool_down,
                skill_fight_relateds::attack_power,
                skill_fight_relateds::mana_power,
                skill_fight_relateds::probability,
                skill_fight_relateds::level,
                skill_fight_relateds::level_experience,
            ))
            .get_results(conn)
    }
}

impl BinaryEncode for FrontDisplaySkill {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        let thumbnail = self.thumbnail.as_bytes();
        encoded.write_i32::<LittleEndian>(thumbnail.len() as i32)?;
        encoded.extend_from_slice(thumbnail);
        let skill_name = self.skill_name.as_bytes();
        encoded.write_i32::<LittleEndian>(skill_name.len() as i32)?;
        encoded.extend_from_slice(skill_name);
        let skill_description = self.skill_description.as_bytes();
        encoded.write_i32::<LittleEndian>(skill_description.len() as i32)?;
        encoded.extend_from_slice(skill_description);
        encoded.write_i32::<LittleEndian>(self.cool_down)?;
        encoded.write_i32::<LittleEndian>(self.attack_power)?;
        encoded.write_i32::<LittleEndian>(self.mana_power)?;
        encoded.write_i16::<LittleEndian>(self.probability)?;
        encoded.write_i16::<LittleEndian>(self.level)?;
        encoded.write_i32::<LittleEndian>(self.level_experience)?;

        //set item length
        encoded.encode()
    }
}
