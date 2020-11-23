use super::redis_db::get_redis_connection;
use crate::{BinaryEncode, DbConnPool, DieselPool};
use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, WriteBytesExt};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::{boxed::Box, collections::HashMap};
use tokio::net::udp::SendHalf;
use tokio::sync::Mutex;
use tracing::info;

type ExecFuture = Pin<Box<dyn Future<Output = ResponseResult> + Send + Sync + 'static>>;
type BoxFn = Box<dyn Fn(ReqContext) -> ExecFuture + Send + Sync + 'static>;

//router register
pub struct RouterRegister {
    route: HashMap<u16, BoxFn>,
}

#[derive(Debug)]
pub struct RouterError(String);

impl RouterRegister {
    pub fn new() -> Self {
        RouterRegister {
            route: HashMap::new(),
        }
    }

    pub fn add<F, R>(&mut self, code: u16, callback: F)
    where
        F: Fn(ReqContext) -> R + Send + Sync + 'static,
        R: Future<Output = ResponseResult> + Send + Sync + 'static,
    {
        self.route
            .insert(code, Box::new(move |req| Box::pin(callback(req))));
    }

    pub fn call(&self, code: u16) -> Result<&BoxFn> {
        let m = match self.route.get(&code) {
            Some(m) => m,
            None => return Err(anyhow!("No code found for {:?}", code)),
        };
        Ok(m)
    }
}

pub type ResponseResult = Result<Vec<u8>>;

#[derive(Clone)]
pub struct ReqContext {
    pub socket: Arc<Mutex<SendHalf>>,
    pub peer: SocketAddr,
    pub db: Arc<DieselPool>,
    pub state: Arc<Mutex<u64>>,
    pub code: u16,
    pub version: u8,
    pub session_id: u64,
    pub body_length: u32,
    pub body: Vec<u8>,
}

impl ReqContext {
    pub fn db_conn(&self) -> Result<DbConnPool> {
        let conn = self.db.get()?;
        Ok(conn)
    }

    pub fn redis_conn(&self) -> Result<redis::Connection> {
        let conn = get_redis_connection()?;
        Ok(conn)
    }

    pub fn get_json<'a, T: Serialize + BinaryEncode + std::fmt::Debug>(
        &self,
        state: u16,
        msg: &'a str,
        body: T,
    ) -> Result<Vec<u8>> {
        let mut gz = ZlibEncoder::new(Vec::new(), Compression::default());
        let mut resp = vec![];

        let res_content = ResponseContext { msg, body };

        let res_body = serde_json::to_vec(&res_content)?;

        resp.write_u16::<LittleEndian>(self.code)?;
        resp.write_u64::<LittleEndian>(self.session_id)?;
        resp.write_u16::<LittleEndian>(state)?;
        resp.write_u32::<LittleEndian>(res_body.len() as u32)?;

        gz.write_all(&res_body)?;

        let gz_resp = gz.finish()?;

        resp.write_u32::<LittleEndian>(gz_resp.len() as u32)?;

        resp.extend_from_slice(&gz_resp);

        Ok(resp)
    }
    pub fn get_bincode<'a, T: BinaryEncode + std::fmt::Debug>(
        &self,
        state: u16,
        msg: &'a str,
        body: T,
    ) -> Result<Vec<u8>> {
        let resp = ResponseContext::get_bincode(self.code, self.session_id, state, msg, body)?;

        Ok(resp)
    }
}

#[allow(unused)]
fn decode_reader(bytes: &[u8]) {
    let mut z = ZlibDecoder::new(bytes);
    let mut s = String::new();
    z.read_to_string(&mut s).unwrap();

    println!("result:P{:?}", s);
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseContext<'a, T: BinaryEncode + std::fmt::Debug> {
    pub msg: &'a str,
    pub body: T,
}

impl<'a, T: BinaryEncode + std::fmt::Debug> ResponseContext<'a, T> {
    pub fn get_bincode(
        code: u16,
        session_id: u64,
        state: u16,
        msg: &'a str,
        body: T,
    ) -> Result<Vec<u8>> {
        let mut gz = ZlibEncoder::new(Vec::new(), Compression::default());
        let mut resp = vec![];

        let mut res_body = Vec::new();
        let msg_bytes = msg.as_bytes();
        res_body.write_i32::<LittleEndian>(msg_bytes.len() as i32)?;
        res_body.extend_from_slice(msg_bytes);

        res_body.extend(body.encode()?);

        info!("Response->res_content:{:?}\n res_body:{:?}", body, res_body);

        resp.write_u16::<LittleEndian>(code)?;
        resp.write_u64::<LittleEndian>(session_id)?;
        resp.write_u16::<LittleEndian>(state)?;
        resp.write_u32::<LittleEndian>(res_body.len() as u32)?;

        gz.write_all(&res_body)?;

        let gz_resp = gz.finish()?;

        resp.write_u32::<LittleEndian>(gz_resp.len() as u32)?;

        resp.extend_from_slice(&gz_resp);

        Ok(resp)
    }
}
