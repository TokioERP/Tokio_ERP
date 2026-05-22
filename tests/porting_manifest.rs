use std::fs;

use tokio_erp::porting::{build_porting_manifest, manifest_to_json, PortStatus};

#[test]
fn builds_manifest_for_python_files_without_runtime_cache_files() {
    let temp = tempfile::tempdir().expect("tempdir");
    let source = temp.path().join("source");
    let target = temp.path().join("target");

    fs::create_dir_all(source.join("accounts/doctype/bank_account_subtype")).expect("source dirs");
    fs::create_dir_all(source.join("accounts/__pycache__")).expect("cache dir");
    fs::write(
        source.join("accounts/doctype/bank_account_subtype/bank_account_subtype.py"),
        "",
    )
    .expect("source file");
    fs::write(source.join("accounts/__pycache__/ignored.pyc"), "").expect("cache file");
    fs::write(source.join("accounts/accounts.json"), "{}").expect("metadata file");

    let manifest = build_porting_manifest(&source, &target).expect("manifest");

    assert_eq!(manifest.entries.len(), 1);
    assert_eq!(
        manifest.entries[0].source,
        "accounts/doctype/bank_account_subtype/bank_account_subtype.py"
    );
    assert_eq!(
        manifest.entries[0].target,
        "accounts/doctype/bank_account_subtype/bank_account_subtype.rs"
    );
    assert_eq!(manifest.entries[0].status, PortStatus::NotStarted);
}

#[test]
fn serializes_manifest_with_stable_status_names() {
    let temp = tempfile::tempdir().expect("tempdir");
    let source = temp.path().join("source");
    let target = temp.path().join("target");

    fs::create_dir_all(source.join("accounts/doctype/finance_book")).expect("source dirs");
    fs::write(
        source.join("accounts/doctype/finance_book/finance_book.py"),
        "",
    )
    .expect("source file");

    let manifest = build_porting_manifest(&source, &target).expect("manifest");
    let json = manifest_to_json(&manifest).expect("json");

    assert!(json.contains("\"source\": \"accounts/doctype/finance_book/finance_book.py\""));
    assert!(json.contains("\"target\": \"accounts/doctype/finance_book/finance_book.rs\""));
    assert!(json.contains("\"status\": \"not_started\""));
}
