use std::fs;

use tokio_erp::porting::mirror_directory_tree;

#[test]
fn mirrors_directory_tree_without_runtime_cache_dirs() {
    let temp = tempfile::tempdir().expect("tempdir");
    let source = temp.path().join("source");
    let target = temp.path().join("target");

    fs::create_dir_all(source.join("accounts/doctype/sales_invoice")).expect("source dirs");
    fs::create_dir_all(source.join("accounts/__pycache__")).expect("cache dir");
    fs::write(source.join("accounts/general_ledger.py"), "").expect("source file");

    let created = mirror_directory_tree(&source, &target).expect("mirror succeeds");

    assert!(target.join("accounts").is_dir());
    assert!(target.join("accounts/.gitkeep").is_file());
    assert!(target.join("accounts/doctype").is_dir());
    assert!(target.join("accounts/doctype/.gitkeep").is_file());
    assert!(target.join("accounts/doctype/sales_invoice").is_dir());
    assert!(target
        .join("accounts/doctype/sales_invoice/.gitkeep")
        .is_file());
    assert!(!target.join("accounts/__pycache__").exists());
    assert!(!target.join("accounts/general_ledger.py").exists());
    assert_eq!(created, 3);
}
