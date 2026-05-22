use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::Serialize;

const SKIPPED_DIRS: &[&str] = &["__pycache__", "node_modules"];

pub fn mirror_directory_tree(source: &Path, target: &Path) -> io::Result<usize> {
    let mut created = 0;
    mirror_directory_tree_inner(source, source, target, &mut created)?;
    Ok(created)
}

fn mirror_directory_tree_inner(
    root: &Path,
    current: &Path,
    target: &Path,
    created: &mut usize,
) -> io::Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() || should_skip_dir(&path) {
            continue;
        }

        let relative = path.strip_prefix(root).map_err(io::Error::other)?;
        let target_dir = target.join(relative);
        fs::create_dir_all(&target_dir)?;
        fs::write(target_dir.join(".gitkeep"), "")?;
        *created += 1;

        mirror_directory_tree_inner(root, &path, target, created)?;
    }

    Ok(())
}

fn should_skip_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| SKIPPED_DIRS.contains(&name))
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct PortingManifest {
    pub entries: Vec<PortingEntry>,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct PortingEntry {
    pub source: String,
    pub target: String,
    pub status: PortStatus,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PortStatus {
    NotStarted,
}

pub fn manifest_to_json(manifest: &PortingManifest) -> serde_json::Result<String> {
    serde_json::to_string_pretty(manifest)
}

pub fn build_porting_manifest(source: &Path, target: &Path) -> io::Result<PortingManifest> {
    let mut entries = Vec::new();
    collect_python_files(source, source, target, &mut entries)?;
    entries.sort_by(|left, right| left.source.cmp(&right.source));
    Ok(PortingManifest { entries })
}

fn collect_python_files(
    root: &Path,
    current: &Path,
    target: &Path,
    entries: &mut Vec<PortingEntry>,
) -> io::Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if !should_skip_dir(&path) {
                collect_python_files(root, &path, target, entries)?;
            }
            continue;
        }

        if path.extension().and_then(|extension| extension.to_str()) != Some("py") {
            continue;
        }

        let relative = path.strip_prefix(root).map_err(io::Error::other)?;
        entries.push(PortingEntry {
            source: path_to_slash_string(relative),
            target: path_to_slash_string(&target_path_for_python_file(target, relative)?),
            status: PortStatus::NotStarted,
        });
    }

    Ok(())
}

fn target_path_for_python_file(_target: &Path, relative: &Path) -> io::Result<PathBuf> {
    let mut path = relative.to_path_buf();
    path.set_extension("rs");
    Ok(path)
}

fn path_to_slash_string(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
