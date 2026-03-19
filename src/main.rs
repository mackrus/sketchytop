use clap::Parser;
use std::process::Command;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
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

    /// Network item name in SketchyBar
    #[arg(long, default_value = "network.latency")]
    net_item: String,

    /// Ping target
    #[arg(long, default_value = "1.1.1.1")]
    ping_target: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut sys = System::new_all();

    let client = Client::new(&Config::default()).unwrap();
    let payload = [0u8; 56];
    let mut pinger = client
        .pinger(args.ping_target.parse().unwrap(), PingIdentifier(0))
        .await;

    loop {
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let cpu_usage = sys.global_cpu_usage();

        let total_mem = sys.total_memory();
        let used_mem = sys.used_memory();
        let mem_perc = (used_mem as f32 / total_mem as f32) * 100.0;
        let used_gb = used_mem as f32 / 1024.0 / 1024.0 / 1024.0;

        // Perform ping
        let mut latency_str = "N/A".to_string();
        let mut net_color = "0xfff39660"; // Orange default for N/A

        if let Ok((IcmpPacket::V4(_), duration)) = pinger.ping(PingSequence(0), &payload).await {
            let latency = duration.as_millis();
            latency_str = format!("{}ms", latency);
            net_color = if latency > 100 {
                "0xfffc5d7c" // Red
            } else if latency > 50 {
                "0xfff39660" // Orange
            } else {
                "0xff9ed072" // Green
            };
        }

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
                "--set",
                &args.net_item,
                &format!("label={}", latency_str),
                &format!("label.color={}", net_color),
            ])
            .output();

        sleep(Duration::from_millis(args.interval)).await;
    }
}
