use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct ZramSubSystem<'a> {
    pub devices: Vec<ZramDevice<'a>>,
}

#[derive(Debug)]
pub struct ZramDevice<'a> {
    pub id: u8,
    pub path: PathBuf,
    pub is_active: bool,
    pub algo: &'a str,
    pub size: Option<String>,
    pub mem_used: Option<u32>,
    pub mem_free: Option<u32>,
}
