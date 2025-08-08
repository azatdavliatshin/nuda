use anyhow::{Context, Result};
use log::{debug, info};
use std::fs;
use std::path::Path;

pub fn copy_template_file(from: &Path, to: &Path, name: &str) -> Result<()> {
    let contents = fs::read_to_string(from)
        .with_context(|| format!("Failed to read template: {}", from.display()))?;
    let contents = contents.replace("{{project_name}}", name);

    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create dir: {}", parent.display()))?;
    }

    fs::write(to, contents).with_context(|| format!("Failed to write file: {}", to.display()))?;

    Ok(())
}

pub fn copy_dir_recursive(from: &Path, to: &Path, project_name: &str) -> Result<()> {
    for entry in walkdir::WalkDir::new(from) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let rel_path = path.strip_prefix(from)?;
            let target_path = to.join(rel_path);

            debug!("Copying: {} → {}", path.display(), target_path.display());

            copy_template_file(path, &target_path, project_name)?;
            info!("✅ Created {}", target_path.display());
        }
    }
    Ok(())
}
