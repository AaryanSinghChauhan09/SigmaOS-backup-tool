mod config;
mod error;
mod service;
mod backup;
mod system;

use config::Config;
use service::Service;
use backup::BackupJobManager;
use system::{SystemMonitor, SystemResource, ResourceStatus};
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    let config = Config::default();
    let mut service = Service::new(config);

    service.start()?;
    println!("Service started successfully");

    // Demonstrate backup management
    let _bm = BackupJobManager::new();
    println!("Backup job manager initialized");

    // Demonstrate system monitoring
    let mut monitor = SystemMonitor::new();
    let resource = SystemResource {
        name: "CPU".to_string(),
        usage: 50.0,
        status: ResourceStatus::Normal,
    };
    monitor.add_resource(resource);
    println!("System health: {}", monitor.check_health());

    service.stop()?;
    println!("Service stopped successfully");

    Ok(())
}
