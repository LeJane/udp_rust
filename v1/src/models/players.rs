use crate::schema::players;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(pid)]
pub struct Player {
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
    pub is_default: i16,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "players"]
pub struct NewPlayer<'a> {
    pub pid: i64,
    pub player_name: &'a str,
    pub model_path: &'a str,
    pub thumbnail: &'a str,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub animation_hit_delay: f32,
    pub spawn_style_class: &'a str,
    pub level: i16,
    pub star_level: i16,
    pub level_experience: i32,
    pub is_default: i16,
}

impl Player {
    pub fn get_is_default_player(conn: &PgConnection, is_default: i16) -> QueryResult<Player> {
        players::table
            .filter(players::is_default.eq(is_default))
            .get_result(conn)
    }
}
