use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PageAndId {
    pub page: u16,
    pub id: u64,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Id {
    pub id: u64,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PlayerIdAndUid {
    pub uid: u64,
    pub pid: u64,
}

pub trait BinaryEncode {
    fn encode(&self) -> Result<Vec<u8>>;
}

//empty str->"".
impl BinaryEncode for &str {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        encoded.write_i32::<LittleEndian>(0)?;

        Ok(encoded)
    }
}

impl BinaryEncode for Vec<u8> {
    fn encode(&self) -> Result<Vec<u8>> {
        let encoded_length = self.len();

        let mut data = Vec::with_capacity(encoded_length + 4);

        data.write_i32::<LittleEndian>(encoded_length as i32)?;

        data.extend(self);

        Ok(data)
    }
}

impl<T: BinaryEncode> BinaryEncode for Vec<T> {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        self.into_iter()
            .map(|e| e.encode().and_then(|v| Ok(encoded.extend_from_slice(&v))))
            .count();

        let mut res = Vec::with_capacity(encoded.len() + 4);

        res.write_i32::<LittleEndian>(encoded.len() as i32)?;

        res.extend_from_slice(&encoded);

        Ok(res)
    }
}
