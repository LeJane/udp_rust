use crate::schema::user_player_tracks;
use crate::{get_guid_value, BinaryEncode};
use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(tid)]
pub struct UserPlayerTrack {
    pub tid: i64,
    pub pid: i64,
    pub uid: i64,
    pub rotaion_x: f32,
    pub rotaion_y: f32,
    pub rotaion_z: f32,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}
#[derive(Debug, Clone, Queryable)]
pub struct FrontDisplayUserPlayerTrack {
    pub tid: i64,
    pub rotaion_x: f32,
    pub rotaion_y: f32,
    pub rotaion_z: f32,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "user_player_tracks"]
pub struct NewUserPlayerTrack {
    pub tid: i64,
    pub pid: i64,
    pub uid: i64,
    pub rotaion_x: f32,
    pub rotaion_y: f32,
    pub rotaion_z: f32,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
}

impl UserPlayerTrack {
    pub fn insert_or_update_player_track(
        conn: &PgConnection,
        pid: i64,
        uid: i64,
        rotaion_x: f32,
        rotaion_y: f32,
        rotaion_z: f32,
        location_x: f32,
        location_y: f32,
        location_z: f32,
    ) -> QueryResult<()> {
        let data = NewUserPlayerTrack {
            tid: get_guid_value() as i64,
            pid,
            uid,
            rotaion_x,
            rotaion_y,
            rotaion_z,
            location_x,
            location_y,
            location_z,
        };

        match user_player_tracks::table
            .filter(user_player_tracks::pid.eq(pid))
            .filter(user_player_tracks::uid.eq(uid))
            .select(user_player_tracks::tid)
            .first::<i64>(conn)
        {
            Ok(v) => {
                diesel::update(user_player_tracks::table)
                    .set((
                        user_player_tracks::rotaion_x.eq(rotaion_x),
                        user_player_tracks::rotaion_y.eq(rotaion_y),
                        user_player_tracks::rotaion_z.eq(rotaion_z),
                        user_player_tracks::location_x.eq(location_x),
                        user_player_tracks::location_y.eq(location_y),
                        user_player_tracks::location_z.eq(location_z),
                    ))
                    .filter(user_player_tracks::tid.eq(v))
                    .execute(conn)?;
            }
            Err(ref e) if e == &diesel::NotFound => {
                diesel::insert_into(user_player_tracks::table)
                    .values(data)
                    .execute(conn)?;
            }
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn get_player_track(
        conn: &PgConnection,
        pid: i64,
        uid: i64,
    ) -> QueryResult<FrontDisplayUserPlayerTrack> {
        user_player_tracks::table
            .filter(user_player_tracks::pid.eq(pid))
            .filter(user_player_tracks::uid.eq(uid))
            .select((
                user_player_tracks::tid,
                user_player_tracks::rotaion_x,
                user_player_tracks::rotaion_y,
                user_player_tracks::rotaion_z,
                user_player_tracks::location_x,
                user_player_tracks::location_y,
                user_player_tracks::location_z,
            ))
            .get_result(conn)
    }
}

impl BinaryEncode for FrontDisplayUserPlayerTrack {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        encoded.write_i64::<LittleEndian>(self.tid)?;
        encoded.write_f32::<LittleEndian>(self.rotaion_x)?;
        encoded.write_f32::<LittleEndian>(self.rotaion_y)?;
        encoded.write_f32::<LittleEndian>(self.rotaion_z)?;
        encoded.write_f32::<LittleEndian>(self.location_x)?;
        encoded.write_f32::<LittleEndian>(self.location_y)?;
        encoded.write_f32::<LittleEndian>(self.location_z)?;

        //set item length
        encoded.encode()
    }
}
