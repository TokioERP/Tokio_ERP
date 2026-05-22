use std::fs;
use std::process::Command;

#[test]
fn generate_manifest_cli_writes_python_file_mapping() {
    let temp = tempfile::tempdir().expect("tempdir");
    let source = temp.path().join("source");
    let target = temp.path().join("target");
    let output_path = temp.path().join("porting_manifest.json");

    fs::create_dir_all(source.join("accounts/doctype/tax_category")).expect("source dirs");
    fs::write(
        source.join("accounts/doctype/tax_category/tax_category.py"),
        "",
    )
    .expect("source file");

    let binary = std::env::var("CARGO_BIN_EXE_generate_porting_manifest")
        .expect("generate manifest cli path");

    let output = Command::new(binary)
        .arg(&source)
        .arg(&target)
        .arg(&output_path)
        .output()
        .expect("run manifest cli");

    assert!(output.status.success());

    let manifest = fs::read_to_string(output_path).expect("manifest file");
    assert!(manifest.contains("accounts/doctype/tax_category/tax_category.py"));
    assert!(manifest.contains("accounts/doctype/tax_category/tax_category.rs"));
}
