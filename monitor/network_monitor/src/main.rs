use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use pcap::{Device, Capture};
use sysinfo::{NetworkExt, System, SystemExt};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Monitor {
        #[arg(short, long)]
        interface: Option<String>,

        #[arg(short, long)]
        output: Option<String>,
    }
}

#[derive(Clone)]
struct NetworkStats {
    rx_bytes: u64,
    tx_bytes: u64,
    rx_packets: u64,
    tx_packets: u64,
    connections: Vec<Connection>,
}

#[derive(Clone)]
struct Connection {
    source: String,
    destination: String,
    protocol: String,
    bytes: u64,
}

struct NetworkMonitor {
    stats: Arc<Mutex<NetworkStats>>,
    caputure:Option<Capture<pcap::Active>>,
    system: System,
}

impl NetworkMonitor {
    fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(NetworkStats {
                rx_bytes: 0,
                tx_bytes: 0,
                rx_packets: 0,
                tx_packets: 0,
                connections: Vec::new()
            })),
            caputure: None,
            system: System::new_all(),
        }
    }

    async fn start_capture(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn monitor(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut network_monitor = NetworkMonitor::new();
    network_monitor.monitor().await?;
    network_monitor.start_capture().await?;

    match cli.command {
        Some(Commands::Monitor { interface, output }) => {
            println!("Monitor command");
            println!("Interface: {:?}", interface);
            println!("Output: {:?}", output);
        }
        None => {
            println!("Please specify a command. Use --help for more information.");
        }
    }

    Ok(())
}
