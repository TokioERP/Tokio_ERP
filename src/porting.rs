use std::fs;
use std::io;
use std::path::Path;

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
