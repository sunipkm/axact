use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WsDataIn {
    pub id: u32,
    pub name: String,
    pub message: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct WsMessage {
    pub id: u32,
    pub name: String,
    pub message: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct MemoryData {
    total: u64,
    free: u64,
    available: u64,
    used: u64,
}

impl MemoryData {
    pub fn new(total: u64, free: u64, available: u64, used: u64) -> Self {
        MemoryData {
            total,
            free,
            available,
            used,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct WsData {
    hostname: String,
    datetime: String,
    ws_count: u32,
    cpu_data: Vec<(u32, f32)>,
    mem_data: MemoryData,
    message: Option<WsMessage>,
}

impl WsData {
    pub fn new(
        hostname: String,
        datetime: String,
        ws_count: u32,
        cpu_data: Vec<(u32, f32)>,
        mem_data: MemoryData,
        message: Option<WsMessage>,
    ) -> Self {
        WsData {
            hostname,
            datetime,
            ws_count,
            cpu_data,
            mem_data,
            message,
        }
    }

    pub fn set_ws_count(self: &mut Self, n: u32) {
        self.ws_count = n;
    }

    pub fn set_message(self: &mut Self, msg: Option<WsMessage>) {
        self.message = msg;
    }
}

pub type Snapshot = WsData;

#[derive(Clone, Debug, Serialize)]
pub struct WsDataOut {
    hostname: String,
    datetime: String,
    ws_count: u32,
    ws_id: u32,
    ws_username: String,
    cpu_data: Vec<(u32, f32)>,
    mem_data: MemoryData,
    message: Option<WsMessage>,
}

impl From<WsData> for WsDataOut {
    fn from(base: WsData) -> Self {
        WsDataOut {
            hostname: base.hostname,
            datetime: base.datetime,
            ws_count: base.ws_count,
            ws_id: 0,
            ws_username: "".to_string(),
            cpu_data: base.cpu_data,
            mem_data: base.mem_data,
            message: base.message,
        }
    }
}

impl WsDataOut {
    pub fn new(base: WsData, id: u32, username: String) -> Self {
        let mut res = WsDataOut::from(base);
        res.ws_id = id;
        res.ws_username = username;

        res
    }
}
