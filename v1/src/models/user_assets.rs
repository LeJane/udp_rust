use crate::get_guid_value;
use crate::schema::user_assets;
use crate::BinaryEncode;
use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, Identifiable, Queryable, Associations)]
#[primary_key(asid)]
pub struct UserAsset {
    pub asid: i64,
    pub uid: i64,
    pub golds: i32,
    pub diamonds: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Queryable)]
pub struct FrontDisplayUserAsset {
    pub golds: i32,
    pub diamonds: i32,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "user_assets"]
pub struct NewUserAsset {
    pub asid: i64,
    pub uid: i64,
    pub golds: i32,
    pub diamonds: i32,
}

impl UserAsset {
    pub fn create_user_assets(conn: &PgConnection, uid: i64) -> QueryResult<()> {
        let asid = get_guid_value() as i64;
        let asset = NewUserAsset {
            asid,
            uid,
            golds: 0,
            diamonds: 0,
        };

        let _usize = diesel::insert_into(user_assets::table)
            .values(asset)
            .execute(conn)?;

        Ok(())
    }

    pub fn get_user_assets(conn: &PgConnection, uid: i64) -> QueryResult<FrontDisplayUserAsset> {
        user_assets::table
            .filter(user_assets::uid.eq(uid))
            .select((user_assets::golds, user_assets::diamonds))
            .first(conn)
    }

    pub fn update_user_assets(
        conn: &PgConnection,
        uid: i64,
        golds: i32,
        diamonds: i32,
    ) -> Result<()> {
        let (old_golds, old_diamond) = user_assets::table
            .filter(user_assets::uid.eq(uid))
            .select((user_assets::golds, user_assets::diamonds))
            .first::<(i32, i32)>(conn)?;

        if golds < 0 && (old_golds + golds) < 0 {
            return Err(anyhow!("user golds not enough."));
        }
        if diamonds < 0 && (old_diamond + diamonds) < 0 {
            return Err(anyhow!("user golds not enough."));
        }

        diesel::update(user_assets::table)
            .set((
                user_assets::golds.eq(user_assets::golds + golds),
                user_assets::diamonds.eq(user_assets::diamonds + diamonds),
            ))
            .filter(user_assets::uid.eq(uid))
            .execute(conn)?;

        Ok(())
    }
}

impl BinaryEncode for FrontDisplayUserAsset {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        encoded.write_i32::<LittleEndian>(self.golds)?;
        encoded.write_i32::<LittleEndian>(self.diamonds)?;

        //set item length
        encoded.encode()
    }
}
