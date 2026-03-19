use clap::Parser;
use std::process::Command;
use sysinfo::System;
use tokio::time::{Duration, sleep};

/// SketchyBar Btop-style system monitor in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Update interval in milliseconds
    #[arg(short, long, default_value_t = 1000)]
    interval: u64,

    /// RAM item name in SketchyBar
    #[arg(long, default_value = "ram")]
    ram_item: String,

    /// CPU item name in SketchyBar
    #[arg(long, default_value = "cpu.percent")]
    cpu_item: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut sys = System::new_all();

    loop {
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let cpu_usage = sys.global_cpu_usage();

        let total_mem = sys.total_memory();
        let used_mem = sys.used_memory();
        let mem_perc = (used_mem as f32 / total_mem as f32) * 100.0;
        let used_gb = used_mem as f32 / 1024.0 / 1024.0 / 1024.0;

        let cpu_color = if cpu_usage > 80.0 {
            "0xfffc5d7c"
        } else if cpu_usage > 50.0 {
            "0xfff39660"
        } else {
            "0xff9ed072"
        };
        let mem_color = if mem_perc > 80.0 {
            "0xfffc5d7c"
        } else if mem_perc > 50.0 {
            "0xfff39660"
        } else {
            "0xff76cce0"
        };

        let _ = Command::new("sketchybar")
            .args([
                "--set",
                &args.cpu_item,
                &format!("label={:.0}%", cpu_usage),
                &format!("label.color={}", cpu_color),
                "--set",
                &args.ram_item,
                &format!("label={:.1}GB", used_gb),
                &format!("label.color={}", mem_color),
            ])
            .output();

        sleep(Duration::from_millis(args.interval)).await;
    }
}
