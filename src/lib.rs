#[derive(Debug, Clone)]
pub struct UserBackupManifest {
    pub home_subdirs: Vec<String>,
    pub export_package_list: bool,
}

pub struct BackupToolRunner;
impl BackupToolRunner {
    pub fn validate_manifest(m: &UserBackupManifest) -> bool {
        !m.home_subdirs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_backup_tool_manifest() {
        let manifest = UserBackupManifest {
            home_subdirs: vec!["Documents".into(), "Pictures".into()],
            export_package_list: true,
        };
        assert!(BackupToolRunner::validate_manifest(&manifest));
    }
}
