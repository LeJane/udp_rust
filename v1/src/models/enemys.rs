use super::skill_fight_relateds::{FrontDisplaySkill, SkillFightRelated};
use crate::schema::enemys;
use crate::BinaryEncode;
use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(eid)]
pub struct Enemy {
    pub eid: i64,
    pub enemy_name: String,
    pub model_path: String,
    pub thumbnail: String,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub animation_hit_delay: f32,
    pub spawn_style_class: String,
    pub bp_enemy: String,
    pub ap_enemy: String,
    pub skm_enemy: String,
    pub aenemy_die: String,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct FrontDisplayEnemy {
    pub eid: i64,
    pub enemy_name: String,
    pub model_path: String,
    pub thumbnail: String,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub animation_hit_delay: f32,
    pub spawn_style_class: String,
    pub bp_enemy: String,
    pub ap_enemy: String,
    pub skm_enemy: String,
    pub aenemy_die: String,
    pub skills: Vec<FrontDisplaySkill>,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "enemys"]
pub struct NewEnemy<'a> {
    pub eid: i64,
    pub enemy_name: &'a str,
    pub model_path: &'a str,
    pub thumbnail: &'a str,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub animation_hit_delay: f32,
    pub spawn_style_class: &'a str,
    pub bp_enemy: &'a str,
    pub ap_enemy: &'a str,
    pub skm_enemy: &'a str,
    pub aenemy_die: &'a str,
}

impl Enemy {
    pub fn get_enemy_data_list(conn: &PgConnection) -> QueryResult<Vec<FrontDisplayEnemy>> {
        let enemy_list = enemys::table.get_results::<Enemy>(conn)?;

        let mut front_display_enemy_data_collection_list = Vec::new();

        for enemy in enemy_list.into_iter() {
            let front_display_skill_list =
                SkillFightRelated::get_front_display_skill_list(conn, enemy.eid, 3)?;

            let front_display_enemy_data_collection = FrontDisplayEnemy {
                eid: enemy.eid,
                enemy_name: enemy.enemy_name,
                model_path: enemy.model_path,
                thumbnail: enemy.thumbnail,
                max_hp: enemy.max_hp,
                attack_power: enemy.attack_power,
                move_speed: enemy.move_speed,
                max_mana: enemy.max_mana,
                defense: enemy.defense,
                animation_hit_delay: enemy.animation_hit_delay,
                spawn_style_class: enemy.spawn_style_class,
                bp_enemy: enemy.bp_enemy,
                ap_enemy: enemy.ap_enemy,
                skm_enemy: enemy.skm_enemy,
                aenemy_die: enemy.aenemy_die,
                skills: front_display_skill_list,
            };

            front_display_enemy_data_collection_list.push(front_display_enemy_data_collection);
        }

        Ok(front_display_enemy_data_collection_list)
    }
}

impl BinaryEncode for FrontDisplayEnemy {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        encoded.write_i64::<LittleEndian>(self.eid)?;
        let enemy_name = self.enemy_name.as_bytes();
        encoded.write_i32::<LittleEndian>(enemy_name.len() as i32)?;
        encoded.extend_from_slice(enemy_name);
        let model_path = self.model_path.as_bytes();
        encoded.write_i32::<LittleEndian>(model_path.len() as i32)?;
        encoded.extend_from_slice(model_path);
        let thumbnail = self.thumbnail.as_bytes();
        encoded.write_i32::<LittleEndian>(thumbnail.len() as i32)?;
        encoded.extend_from_slice(thumbnail);
        encoded.write_i32::<LittleEndian>(self.max_hp)?;
        encoded.write_i32::<LittleEndian>(self.attack_power)?;
        encoded.write_f32::<LittleEndian>(self.move_speed)?;
        encoded.write_i32::<LittleEndian>(self.max_mana)?;
        encoded.write_i32::<LittleEndian>(self.defense)?;
        encoded.write_f32::<LittleEndian>(self.animation_hit_delay)?;
        let spawn_style_class = self.spawn_style_class.as_bytes();
        encoded.write_i32::<LittleEndian>(spawn_style_class.len() as i32)?;
        encoded.extend_from_slice(spawn_style_class);
        let bp_enemy = self.bp_enemy.as_bytes();
        encoded.write_i32::<LittleEndian>(bp_enemy.len() as i32)?;
        encoded.extend_from_slice(bp_enemy);
        let ap_enemy = self.ap_enemy.as_bytes();
        encoded.write_i32::<LittleEndian>(ap_enemy.len() as i32)?;
        encoded.extend_from_slice(ap_enemy);
        let skm_enemy = self.skm_enemy.as_bytes();
        encoded.write_i32::<LittleEndian>(skm_enemy.len() as i32)?;
        encoded.extend_from_slice(skm_enemy);
        let aenemy_die = self.aenemy_die.as_bytes();
        encoded.write_i32::<LittleEndian>(aenemy_die.len() as i32)?;
        encoded.extend_from_slice(aenemy_die);

        //skills
        let skills = self.skills.encode()?;

        encoded.extend(skills);

        //set item length
        encoded.encode()
    }
}
