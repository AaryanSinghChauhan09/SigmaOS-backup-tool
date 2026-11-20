//! Integration tests for SigmaOS-backup-tool.

use sigmaos_backup_tool_lib::{Component, ComponentLifecycle};

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
