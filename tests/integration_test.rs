use anyhow::Result;
use file_manager::{Component, ComponentStatus, Config, FileManager};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_manager_lifecycle_aaa() -> Result<()> {
        let config = Config::default();
        let mut fm = FileManager::new(config);
        
        assert_eq!(fm.status(), ComponentStatus::Stopped);
        fm.initialize()?;
        fm.start()?;
        assert_eq!(fm.status(), ComponentStatus::Running);
        fm.stop()?;
        assert_eq!(fm.status(), ComponentStatus::Stopped);
        Ok(())
    }

    #[test]
    fn test_file_manager_metadata_aaa() {
        let config = Config::default();
        let fm = FileManager::new(config);
        assert_eq!(fm.name(), "SigmaOS-file-manager");
        assert_eq!(fm.version(), "0.1.0");
    }
}
