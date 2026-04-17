use std::path::PathBuf;

use crate::algo::Algo;

#[derive(Debug, Default)]
pub struct ZramSubSystem {
    pub devices: Vec<ZramDevice>,
}

#[derive(Debug)]
pub struct ZramDevice {
    pub id: u8,
    pub path: PathBuf,
    pub is_active: bool,
    pub algo: Algo,
    pub size: Option<String>,
    pub mem_used: Option<u32>,
    pub mem_free: Option<u32>,
}
