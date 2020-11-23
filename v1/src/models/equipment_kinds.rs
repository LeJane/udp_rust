use crate::schema::equipment_kinds;
use crate::BinaryEncode;
use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(kid)]
pub struct EquipmentKind {
    pub kid: i64,
    pub name: String,
    pub kind: i16, //1:weapons,2:armors,3:shields
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct FrontDisplayEquipmentKind {
    pub kid: i64,
    pub name: String,
    pub kind: i16,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "equipment_kinds"]
pub struct NewEquipmentKind<'a> {
    pub kid: i64,
    pub name: &'a str,
    pub kind: i16,
}

impl EquipmentKind {
    pub fn get_equipment_kind_list(
        conn: &PgConnection,
    ) -> QueryResult<Vec<FrontDisplayEquipmentKind>> {
        equipment_kinds::table
            .select((
                equipment_kinds::kid,
                equipment_kinds::name,
                equipment_kinds::kind,
            ))
            .load(conn)
    }
}

impl BinaryEncode for FrontDisplayEquipmentKind {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        encoded.write_i64::<LittleEndian>(self.kid)?;
        let name = self.name.as_bytes();
        encoded.write_i32::<LittleEndian>(name.len() as i32)?;
        encoded.extend_from_slice(name);
        encoded.write_i16::<LittleEndian>(self.kind)?;

        //set item length
        encoded.encode()
    }
}
