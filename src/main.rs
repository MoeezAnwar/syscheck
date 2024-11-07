use sysinfo::{System, SystemExt, DiskExt, ProcessorExt, NetworkExt};
use chrono::Local;

fn generate_health_report() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Retrieve system information
    let os = sys.name().unwrap_or_else(|| "Unknown OS".to_string());
    let kernel_version = sys.kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let uptime = sys.uptime();
    let load_avg = sys.load_average();
    let cpu_usage = sys.global_processor_info().cpu_usage();
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    // Retrieve disk information
    println!("\n--- Disk Information ---");
    for disk in sys.disks() {
        println!(
            "Disk: {} | Type: {:?} | Total: {} GB | Used: {} GB",
            disk.name().to_string_lossy(),
            disk.type_(),
            disk.total_space() / 1_073_741_824,  // Convert to GB
            disk.used_space() / 1_073_741_824    // Convert to GB
        );
    }

    // Retrieve network information
    println!("\n--- Network Information ---");
    for (interface_name, data) in sys.networks() {
        println!(
            "Interface: {} | Received: {} MB | Transmitted: {} MB",
            interface_name,
            data.received() / 1_048_576,  // Convert to MB
            data.transmitted() / 1_048_576 // Convert to MB
        );
    }

    // Display collected information
    println!("\n--- Laptop Health Report ---");
    println!("Timestamp: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("Operating System: {}", os);
    println!("Kernel Version: {}", kernel_version);
    println!("Uptime: {} seconds", uptime);
    println!("Load Average: {:.2}, {:.2}, {:.2}", load_avg.one, load_avg.five, load_avg.fifteen);
    println!("CPU Usage: {:.2}%", cpu_usage);
    println!("Memory: {} MB used out of {} MB", used_memory / 1024, total_memory / 1024);
    println!("Swap: {} MB used out of {} MB", used_swap / 1024, total_swap / 1024);
}

fn main() {
    println!("Generating laptop health report...");
    generate_health_report();
}
