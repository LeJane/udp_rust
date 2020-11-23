#[macro_use]
extern crate diesel;

pub mod controller;
pub mod models;
pub mod router;
pub mod schema;
pub mod utils;

pub use router::build_routers;

pub use utils::router::{ReqContext, ResponseContext, ResponseResult, RouterRegister};
pub use utils::{
    common::BinaryEncode,
    common::PageAndId,
    diesel_db::{get_diesel_pool, DbConnPool, DieselPool},
    helper::get_guid_value,
    redis_db::get_redis_connection,
    redis_db::GLOBAL_USERS_COUNTER_REDIS_KEY,
    thread_pool::ThreadPool,
};
