use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
struct Manifest {
    entries: Vec<ManifestEntry>,
}

#[derive(Deserialize)]
struct ManifestEntry {
    source: String,
    target: String,
    status: String,
}

#[test]
fn accounts_doctype_empty_init_modules_are_mirrored_and_tracked() {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("porting_manifest.json");
    let manifest_json = fs::read_to_string(manifest_path).expect("manifest");
    let manifest: Manifest = serde_json::from_str(&manifest_json).expect("manifest json");
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace parent")
        .join("erpnext/apps/erpnext/erpnext");

    let mut missing_targets = Vec::new();
    let mut untracked_empty_inits = Vec::new();

    for entry in manifest.entries.iter().filter(|entry| {
        entry.source.starts_with("accounts/doctype/") && entry.source.ends_with("__init__.py")
    }) {
        let source_path = source_root.join(&entry.source);
        let source_is_empty = fs::read_to_string(&source_path)
            .map(|content| content.is_empty())
            .unwrap_or(false);

        if source_is_empty && entry.status == "not_started" {
            untracked_empty_inits.push(entry.source.clone());
        }

        if source_is_empty {
            let target_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src/erpnext")
                .join(&entry.target);
            if !target_path.is_file() {
                missing_targets.push(entry.target.clone());
            }
        }
    }

    assert!(
        untracked_empty_inits.is_empty(),
        "untracked empty init modules: {untracked_empty_inits:#?}"
    );
    assert!(
        missing_targets.is_empty(),
        "missing mirrored init module targets: {missing_targets:#?}"
    );
}

#[test]
fn all_accounts_empty_init_modules_are_mirrored_and_tracked() {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("porting_manifest.json");
    let manifest_json = fs::read_to_string(manifest_path).expect("manifest");
    let manifest: Manifest = serde_json::from_str(&manifest_json).expect("manifest json");
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace parent")
        .join("erpnext/apps/erpnext/erpnext");

    let mut missing_targets = Vec::new();
    let mut untracked_empty_inits = Vec::new();

    for entry in manifest.entries.iter().filter(|entry| {
        entry.source.starts_with("accounts/") && entry.source.ends_with("__init__.py")
    }) {
        let source_path = source_root.join(&entry.source);
        let source_is_empty = fs::read_to_string(&source_path)
            .map(|content| content.is_empty())
            .unwrap_or(false);

        if source_is_empty && entry.status == "not_started" {
            untracked_empty_inits.push(entry.source.clone());
        }

        if source_is_empty {
            let target_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src/erpnext")
                .join(&entry.target);
            if !target_path.is_file() {
                missing_targets.push(entry.target.clone());
            }
        }
    }

    assert!(
        untracked_empty_inits.is_empty(),
        "untracked empty init modules: {untracked_empty_inits:#?}"
    );
    assert!(
        missing_targets.is_empty(),
        "missing mirrored init module targets: {missing_targets:#?}"
    );
}
