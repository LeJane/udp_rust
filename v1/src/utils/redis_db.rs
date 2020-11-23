use redis::{Connection, RedisResult};

pub fn get_redis_connection() -> RedisResult<Connection> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();

    let conn = client.get_connection();

    conn
}

pub const GLOBAL_USERS_COUNTER_REDIS_KEY: &'static str = "global_user_counter";
