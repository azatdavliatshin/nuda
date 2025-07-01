use anyhow::{Context, Result};
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
