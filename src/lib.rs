extern crate libc;
extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct PirQuery {
    #[serde(with = "serde_bytes")] pub query: Vec<u8>,
    pub num: u64,
}

#[derive(Serialize, Deserialize)]
pub struct PirReply {
    #[serde(with = "serde_bytes")] pub reply: Vec<u8>,
    pub num: u64,
}

pub mod client;
pub mod server;
