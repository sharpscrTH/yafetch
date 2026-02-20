use colored::*;
use std::fs;
use std::path::Path;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

fn main() {
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
            .with_memory(MemoryRefreshKind::nothing().with_ram()),
    );

    let user = std::env::var("USER").unwrap_or_else(|_| "user".into());
    let host = System::host_name().unwrap_or_else(|| "localhost".into());
    let os_name = System::name().unwrap_or_else(|| "Linux".into());
    let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".into());
    let uptime_raw = System::uptime();
    let shell = std::env::var("SHELL")
        .map(|s| s.split('/').last().unwrap_or("unknown").to_string())
        .unwrap_or_else(|_| "unknown".into());

    let desktop = std::env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| std::env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| std::env::var("XDG_SESSION_DESKTOP").unwrap_or_else(|_| "none".into()));

    let (pkgs, manager) = if Path::new("/var/lib/pacman/local").exists() {
        (
            fs::read_dir("/var/lib/pacman/local")
                .map(|d| d.count().saturating_sub(1))
                .unwrap_or(0),
            "pacman",
        )
    } else if Path::new("/var/lib/dpkg/status").exists() {
        let count = fs::read_to_string("/var/lib/dpkg/status")
            .unwrap_or_default()
            .lines()
            .filter(|l| l.starts_with("Package:"))
            .count();
        (count, "dpkg")
    } else {
        (0, "unknown")
    };

    let hours = uptime_raw / 3600;
    let minutes = (uptime_raw % 3600) / 60;
    let uptime_str = if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    };

    println!("{}@{}", user.cyan().bold(), host.cyan().bold());
    println!("{}", "------------".bright_black());

    println!("{:<12} {}", "OS".cyan().bold(), os_name);
    println!("{:<12} {}", "Kernel".magenta().bold(), kernel);
    println!("{:<12} {}", "Uptime".white().bold(), uptime_str);

    if desktop != "none" {
        println!("{:<12} {}", "DE/WM".blue().bold(), desktop);
    }

    if pkgs > 0 {
        println!(
            "{:<12} {} ({})",
            "Packages".bright_blue().bold(),
            pkgs,
            manager
        );
    }

    println!("{:<12} {}", "Shell".yellow().bold(), shell);

    let total_ram = sys.total_memory() / 1_048_576;
    let used_ram = sys.used_memory() / 1_048_576;
    println!(
        "{:<12} {} MB / {} MB",
        "Memory".bright_yellow().bold(),
        used_ram,
        total_ram
    );

    if let Some(cpu) = sys.cpus().first() {
        println!("{:<12} {}", "CPU".green().bold(), cpu.brand().trim());
    }
}
