use zram_rs::components::ZramSubSystem;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use ctrlc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("Received Ctrl+C. Shutting down...");
        r.store(false, Ordering::SeqCst);
    })?;

    let mut zram_system = ZramSubSystem::new();
    
    zram_system.add_device();
    zram_system.add_device_with_size(1234);
    zram_system.list_devices();
    
    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    Ok(())
}