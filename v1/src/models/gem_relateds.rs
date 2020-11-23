use super::gems::Gem;
use crate::schema::gem_relateds;
use crate::BinaryEncode;
use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(grid)]
pub struct GemRelated {
    pub grid: i64,
    pub obj_id: i64,
    pub gid: i64,
    pub obj_type: i16,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct FrontDisplayGem {
    pub gem_icon: String,
    pub gem_selected_material: String,
    pub gem_link_material: String,
    pub model_path: String,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "gem_relateds"]
pub struct NewGemRelated {
    pub grid: i64,
    pub obj_id: i64,
    pub gid: i64,
    pub obj_type: i16,
}

impl GemRelated {
    pub fn get_front_display_gem_related_list(
        conn: &PgConnection,
        obj_id: i64,
        obj_type: i16,
    ) -> QueryResult<Vec<FrontDisplayGem>> {
        let front_gem_relateds = gem_relateds::table
            .filter(gem_relateds::obj_id.eq(obj_id))
            .filter(gem_relateds::obj_type.eq(obj_type))
            .get_results::<GemRelated>(conn)?;

        let mut front_display_gem_list = Vec::new();

        for front_gem_related in front_gem_relateds.into_iter() {
            let gem = Gem::get_gem_by_gid(conn, front_gem_related.gid)?;

            let front_display_gem = FrontDisplayGem {
                gem_icon: gem.gem_icon,
                gem_selected_material: gem.gem_selected_material,
                gem_link_material: gem.gem_link_material,
                model_path: gem.model_path,
            };

            front_display_gem_list.push(front_display_gem);
        }

        Ok(front_display_gem_list)
    }
}

impl BinaryEncode for FrontDisplayGem {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        let icon = self.gem_icon.as_bytes();
        encoded.write_i32::<LittleEndian>(icon.len() as i32)?;
        encoded.extend_from_slice(icon);
        let gem_selected_material = self.gem_selected_material.as_bytes();
        encoded.write_i32::<LittleEndian>(gem_selected_material.len() as i32)?;
        encoded.extend_from_slice(gem_selected_material);
        let gem_link_material = self.gem_link_material.as_bytes();
        encoded.write_i32::<LittleEndian>(gem_link_material.len() as i32)?;
        encoded.extend_from_slice(gem_link_material);
        let model_path = self.model_path.as_bytes();
        encoded.write_i32::<LittleEndian>(model_path.len() as i32)?;
        encoded.extend_from_slice(model_path);

        //set item length
        encoded.encode()
    }
}
