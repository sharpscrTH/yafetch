use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!(
        "OS:       {:?}",
        System::name().unwrap_or("Unknown".to_string())
    );
    println!(
        "Kernel:   {:?}",
        System::kernel_version().unwrap_or("Unknown".to_string())
    );
    println!(
        "Host:     {:?}",
        System::host_name().unwrap_or("Unknown".to_string())
    );

    let total_ram = sys.total_memory() / 1024 / 1024;
    let used_ram = sys.used_memory() / 1024 / 1024;
    println!("Memory:   {} MB / {} MB", used_ram, total_ram);

    if let some_cpu = sys.cpus().first() {
        println!("CPU:      {}", some_cpu.expect("REASON").brand());
    }
}
