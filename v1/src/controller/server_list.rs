use crate::models::server_lists::ServerList;
use crate::{ReqContext, ResponseResult};
use byteorder::{LittleEndian, ReadBytesExt};

pub async fn get_server_list(ctx: ReqContext) -> ResponseResult {
    let diesel_conn = ctx.db_conn()?;

    let mut cursor = std::io::Cursor::new(&ctx.body);
    let x = cursor.read_i32::<LittleEndian>()?;

    println!("x:{}", x);
    let server_lists = ServerList::get_server_list_ip_port(&diesel_conn)?;

    ctx.get_bincode(200, "Success", server_lists)
}
