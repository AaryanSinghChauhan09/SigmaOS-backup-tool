use sigmaos_system_component::{Config, Service, BackupJobManager};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default_aaa() {
        // Arrange & Act
        let config = Config::default();
        // Assert
        assert!(config.process().is_ok());
    }

    #[test]
    fn test_service_lifecycle_aaa() {
        // Arrange
        let config = Config::default();
        let mut service = Service::new(config);
        // Act
        assert!(!service.is_running());
        assert!(service.start().is_ok());
        assert!(service.is_running());
        // Assert
        assert!(service.stop().is_ok());
        assert!(!service.is_running());
    }

    #[test]
    fn test_backup_job_manager_new_aaa() {
        // Arrange & Act
        let bm = BackupJobManager::new();
        // Assert: default sources are pre-populated
        assert!(!bm.list_sources().is_empty());
    }
}
