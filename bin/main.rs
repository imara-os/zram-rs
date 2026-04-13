fn main() {
    let mut zram_subsystem = zram_rs::ZramSubSystem::new();
    for n in 0..4 {
        zram_subsystem.add_device();
        println!("Added new zram device with id: {}", zram_subsystem.devices[n].id);
    }
    zram_subsystem.list_devices();
    
    zram_subsystem.remove_all_devices();
    println!("Removed all zram devices");
    zram_subsystem.list_devices();    
}
