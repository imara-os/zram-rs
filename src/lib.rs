use std::fs;
use std::path::Path;

const ZRAM_DEVICE_ADD_PATH: &str = "/sys/class/zram-control/hot_add";
const ZRAM_DEVICE_REMOVE_PATH: &str = "/sys/class/zram-control/hot_remove";

#[derive(Default)]
pub struct ZramSubSystem {
    pub devices: Vec<ZramDevice>,
}

#[derive(Default)]
pub struct ZramDevice {
    pub id: u8,
    pub is_active: bool,
}

impl ZramDevice {
    pub fn new() -> Self {
        let zram_device_id = fs::read_to_string(Path::new(ZRAM_DEVICE_ADD_PATH))
            .expect("Failed to open /sys/class/zram-control/hot_add")
            .trim()
            .parse::<u8>()
            .expect("Failed to parse zram device id");

        Self {
            id: zram_device_id,
            is_active: false,
        }
    }
    pub fn remove(&self) {
        fs::write(Path::new(ZRAM_DEVICE_REMOVE_PATH), self.id.to_string())
            .expect("Failed to remove zram device");
    }
}

impl ZramSubSystem {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self) {
        let device = ZramDevice::new();
        self.devices.push(device);
    }
    pub fn remove_device(&mut self, id: u8) {
        self.devices.retain(|device| device.id != id);
        fs::write(Path::new(ZRAM_DEVICE_REMOVE_PATH), id.to_string())
            .expect("Failed to remove zram device");
    }
    pub fn remove_all_devices(&mut self) {
        for device in &self.devices {
            device.remove();
        }
        self.devices.clear();
    }
    pub fn list_devices(&self) {
        if self.devices.is_empty() {
            println!("No zram devices found.");
            return;
        }
        for device in &self.devices {
            println!("Zram device id: {}", device.id);
        }
    }
}
