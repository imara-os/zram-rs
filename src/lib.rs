use std::{fs, path::Path};

use crate::{algo::Algo, components::{ZramDevice, ZramSubSystem}};

pub mod algo;
pub mod components;
pub mod error;

const ZRAM_CONTROL_PATH_HOT_ADD: &str = "/sys/class/zram-control/hot_add";
const ZRAM_CONTROL_PATH_HOT_REMOVE: &str = "/sys/class/zram-control/hot_remove";
const ZRAM_DEVICE_PATH_PREFIX: &str = "/sys/block/";

impl ZramSubSystem {
    pub fn new() -> Self {
        ZramSubSystem {
            devices: Vec::new(),
        }
    }
    pub fn add_device(&mut self) {
        self.devices.push(ZramDevice::default());
    }
    pub fn add_device_with_size(&mut self, size: u32) {
        self.devices.push(ZramDevice::with_size(size));
    }
    pub fn remove_device(&mut self, device: ZramDevice) {
        if device.id < self.devices.len().try_into().unwrap() {
            self.devices.remove(device.id.into());
            device.remove();
        }
    }
    pub fn remove_device_with_id(&mut self, device_id: u8) {
        if device_id < self.devices.len().try_into().unwrap() {
             // let device = self.get_device_with_id(device_id);
            self.get_device_with_id(device_id).unwrap().remove();
            self.devices.remove(device_id.into());
        }
    }
    pub fn list_devices(&self) {
        for device in &self.devices {
            println!("Found zram device: {:?}", device);
        }
    }
    pub fn get_device_with_id(&self, device_id: u8) -> Option<&ZramDevice> {
        self.devices.get(device_id as usize)
    }
}

impl Drop for ZramSubSystem {
    fn drop(&mut self) {
        for device in &mut self.devices {
            device.remove();
        }
    }
}

impl ZramDevice {
    pub fn new() -> Self {
        let device_id = fs::read_to_string(ZRAM_CONTROL_PATH_HOT_ADD)
            .expect("Failed to aquire zram device and corresponding id")
            .trim()
            .parse::<u8>()
            .expect("Failed to parse device id");
        let device_path = Path::new(ZRAM_DEVICE_PATH_PREFIX).join(format!("zram{device_id}"));
        ZramDevice {
            id: device_id,
            path: device_path,
            is_active: false,
            algo: Algo::Zstd,
            size: None,
            mem_used: None,
            mem_free: None,
        }
    }
    pub fn with_size(size: u32) -> Self {
        let device_id = fs::read_to_string(ZRAM_CONTROL_PATH_HOT_ADD)
            .expect("Failed to aquire zram device and corresponding id")
            .trim()
            .parse::<u8>()
            .expect("Failed to parse device id");
        let device_path = Path::new(ZRAM_DEVICE_PATH_PREFIX).join(format!("zram{device_id}"));
        let mut device_size = size.to_string();
        
        device_size.push('M');
        
        ZramDevice {
            id: device_id,
            path: device_path,
            is_active: false,
            algo: Algo::Zstd,
            size: Some(device_size),
            mem_used: None,
            mem_free: None,
        }
    }
    pub fn remove(&self) {
        let id_str = self.id.to_string();
        match fs::write(Path::new(ZRAM_CONTROL_PATH_HOT_REMOVE), id_str.as_bytes()) {
            Err(e) => eprintln!("Could not remove device with id: '{}' at: {:?}. Error: {e}", self.id, self.path),
            Ok(_) => println!("Device with id: '{}' at: {:?} sucessfully removed!", self.id, self.path),
        };
    }
}

impl Default for ZramDevice {
    fn default() -> Self {
        let device_id = fs::read_to_string(ZRAM_CONTROL_PATH_HOT_ADD)
            .expect("Failed to aquire zram device and corresponding id")
            .trim()
            .parse::<u8>()
            .expect("Failed to parse device id");
        let device_path = Path::new(ZRAM_DEVICE_PATH_PREFIX).join(format!("zram{device_id}"));
        ZramDevice {
            id: device_id,
            path: device_path,
            is_active: false,
            algo: Algo::Zstd,
            size: Some(String::from("4096M")),
            mem_used: None,
            mem_free: None,
        }
    }
}
