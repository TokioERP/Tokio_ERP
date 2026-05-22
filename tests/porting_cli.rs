use std::fs;
use std::process::Command;

#[test]
fn mirror_cli_creates_target_tree() {
    let temp = tempfile::tempdir().expect("tempdir");
    let source = temp.path().join("source");
    let target = temp.path().join("target");

    fs::create_dir_all(source.join("accounts/doctype/account")).expect("source dirs");

    let binary = std::env::var("CARGO_BIN_EXE_mirror_erpnext_tree").expect("mirror cli path");

    let output = Command::new(binary)
        .arg(&source)
        .arg(&target)
        .output()
        .expect("run mirror cli");

    assert!(output.status.success());
    assert!(target.join("accounts/doctype/account/.gitkeep").is_file());
}
