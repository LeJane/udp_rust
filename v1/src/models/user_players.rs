use super::{
    equipments::{Equipment, EquipmentData},
    gem_relateds::{FrontDisplayGem, GemRelated},
    player_mount_equipments::PlayerMountEquipment,
    players::Player,
    skill_fight_relateds::{FrontDisplaySkill, SkillFightRelated},
};
use crate::get_guid_value;
use crate::schema::{players, user_players};
use crate::BinaryEncode;
use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Identifiable, Queryable)]
pub struct UserPlayer {
    pub id: i64,
    pub pid: i64,
    pub uid: i64,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub level: i16,
    pub star_level: i16,
    pub level_experience: i32,
    pub is_default: i16, //1:unactived,2:actived
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct FrontDisplayPlayer {
    pub pid: i64,
    pub player_name: String,
    pub model_path: String,
    pub thumbnail: String,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub animation_hit_delay: f32,
    pub spawn_style_class: String,
    pub level: i16,
    pub star_level: i16,
    pub level_experience: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct FrontDisplayPlayerDataCollection {
    pub pid: i64,
    pub player_name: String,
    pub model_path: String,
    pub thumbnail: String,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub animation_hit_delay: f32,
    pub spawn_style_class: String,
    pub level: i16,
    pub star_level: i16,
    pub level_experience: i32,
    pub gems: Vec<FrontDisplayGem>,
    pub skills: Vec<FrontDisplaySkill>,
    pub mount_equipments: Vec<EquipmentData>,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "user_players"]
pub struct NewUserPlayer {
    pub id: i64,
    pub pid: i64,
    pub uid: i64,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub level: i16,
    pub star_level: i16,
    pub level_experience: i32,
    pub is_default: i16,
}

impl UserPlayer {
    pub fn get_user_default_player_data(
        conn: &PgConnection,
        uid: i64,
    ) -> QueryResult<FrontDisplayPlayer> {
        user_players::table
            .filter(user_players::uid.eq(uid))
            .filter(user_players::is_default.eq(2))
            .inner_join(players::table.on(players::pid.eq(user_players::pid)))
            .select((
                user_players::pid,
                players::player_name,
                players::model_path,
                players::thumbnail,
                user_players::max_hp,
                user_players::attack_power,
                user_players::move_speed,
                user_players::max_mana,
                user_players::defense,
                players::animation_hit_delay,
                players::spawn_style_class,
                user_players::level,
                user_players::star_level,
                user_players::level_experience,
            ))
            .get_result(conn)
    }

    pub fn get_player_data_collection_by_pid(
        conn: &PgConnection,
        uid: i64,
        pid: i64,
    ) -> QueryResult<FrontDisplayPlayerDataCollection> {
        let front_display_player: FrontDisplayPlayer = user_players::table
            .filter(user_players::uid.eq(uid))
            .filter(user_players::pid.eq(pid))
            .inner_join(players::table.on(players::pid.eq(user_players::pid)))
            .select((
                user_players::pid,
                players::player_name,
                players::model_path,
                players::thumbnail,
                user_players::max_hp,
                user_players::attack_power,
                user_players::move_speed,
                user_players::max_mana,
                user_players::defense,
                players::animation_hit_delay,
                players::spawn_style_class,
                user_players::level,
                user_players::star_level,
                user_players::level_experience,
            ))
            .get_result(conn)?;

        let front_display_gems = GemRelated::get_front_display_gem_related_list(conn, pid, 1)?;

        let front_display_skills = SkillFightRelated::get_front_display_skill_list(conn, pid, 1)?;

        let equipments = PlayerMountEquipment::get_player_mount_equipment_list(conn, pid, uid)?;

        let player_mount_equipment_data =
            Equipment::get_equipment_data_collection(conn, equipments)?;

        Ok(FrontDisplayPlayerDataCollection {
            pid: front_display_player.pid,
            player_name: front_display_player.player_name,
            model_path: front_display_player.model_path,
            thumbnail: front_display_player.thumbnail,
            max_hp: front_display_player.max_hp,
            attack_power: front_display_player.attack_power,
            move_speed: front_display_player.move_speed,
            max_mana: front_display_player.max_mana,
            defense: front_display_player.defense,
            animation_hit_delay: front_display_player.animation_hit_delay,
            spawn_style_class: front_display_player.spawn_style_class,
            level: front_display_player.level,
            star_level: front_display_player.star_level,
            level_experience: front_display_player.level_experience,
            gems: front_display_gems,
            skills: front_display_skills,
            mount_equipments: player_mount_equipment_data,
        })
    }

    pub fn create_user_default_player(conn: &PgConnection, uid: i64) -> QueryResult<()> {
        //get default player
        let player = Player::get_is_default_player(conn, 2)?;

        let user_player = NewUserPlayer {
            id: get_guid_value() as i64,
            pid: player.pid,
            uid: uid,
            max_hp: player.max_hp,
            attack_power: player.attack_power,
            move_speed: player.move_speed,
            max_mana: player.max_mana,
            defense: player.defense,
            level: player.level,
            star_level: player.star_level,
            level_experience: player.level_experience,
            is_default: 2,
        };

        let _usize = diesel::insert_into(user_players::table)
            .values(user_player)
            .execute(conn)?;

        Ok(())
    }
}

impl BinaryEncode for FrontDisplayPlayer {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        encoded.write_i64::<LittleEndian>(self.pid)?;
        let player_name = self.player_name.as_bytes();
        encoded.write_i32::<LittleEndian>(player_name.len() as i32)?;
        encoded.extend_from_slice(player_name);
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
        encoded.write_i16::<LittleEndian>(self.level)?;
        encoded.write_i16::<LittleEndian>(self.star_level)?;
        encoded.write_i32::<LittleEndian>(self.level_experience)?;

        //set item length
        encoded.encode()
    }
}
impl BinaryEncode for FrontDisplayPlayerDataCollection {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        encoded.write_i64::<LittleEndian>(self.pid)?;
        let player_name = self.player_name.as_bytes();
        encoded.write_i32::<LittleEndian>(player_name.len() as i32)?;
        encoded.extend_from_slice(player_name);
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
        encoded.write_i16::<LittleEndian>(self.level)?;
        encoded.write_i16::<LittleEndian>(self.star_level)?;
        encoded.write_i32::<LittleEndian>(self.level_experience)?;

        //gems
        let gems = self.gems.encode()?;
        encoded.extend(gems);
        //skills
        let skills = self.skills.encode()?;
        encoded.extend(skills);
        //mount_equipments
        let mount_equipments = self.mount_equipments.encode()?;
        encoded.extend(mount_equipments);

        //set item length
        encoded.encode()
    }
}
