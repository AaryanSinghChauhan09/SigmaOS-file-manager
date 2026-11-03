//! SigmaOS-file-manager - SigmaOS Component CLI
//!
//! Command-line interface for SigmaOS-file-manager component.

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;
use file-manager::{Component, ComponentStatus, Config, file-manager};

#[derive(Parser)]
#[command(name = "file-manager")]
#[command(about = "SigmaOS SigmaOS-file-manager component", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the component
    Init {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
    /// Start the component
    Start,
    /// Stop the component
    Stop,
    /// Get component status
    Status,
    /// Run the component
    Run {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let config = Config::default();
    let mut component = file-manager::new(config);

    match cli.command {
        Commands::Init { debug } => {
            if debug {
                println!("Initializing SigmaOS-file-manager in debug mode...");
            } else {
                println!("Initializing SigmaOS-file-manager...");
            }
            component.initialize()?;
            println!("SigmaOS-file-manager initialized successfully");
        }
        Commands::Start => {
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-file-manager started successfully");
        }
        Commands::Stop => {
            component.stop()?;
            println!("SigmaOS-file-manager stopped successfully");
        }
        Commands::Status => {
            let status = component.status();
            println!("SigmaOS-file-manager status: {:?}", status);
        }
        Commands::Run { debug } => {
            if debug {
                println!("Running SigmaOS-file-manager in debug mode...");
            } else {
                println!("Running SigmaOS-file-manager...");
            }
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-file-manager is running. Press Ctrl+C to stop.");
            
            // Keep running
            tokio::runtime::Runtime::new()?.block_on(async {
                tokio::signal::ctrl_c().await.unwrap();
                info!("Received Ctrl+C, shutting down...");
            });
            
            component.stop()?;
            println!("SigmaOS-file-manager stopped");
        }
    }

    Ok(())
}
