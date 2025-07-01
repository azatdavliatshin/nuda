use anyhow::{Context, Result};
use log::{debug, info};
use std::fs;
use std::path::PathBuf;

mod template;

pub fn scaffold_project(name: &str) -> Result<()> {
    let template_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../templates/default");
    let project_root = PathBuf::from(name);
    let copy_plan = vec![
        ("pages/index.html", "pages/index.html"),
        ("public/favicon.ico", "public/favicon.ico"),
        ("nuda.config.json", "nuda.config.json"),
        ("gitignore", ".gitignore"),
        ("README.md", "README.md"),
        ("package.json", "package.json"),
    ];

    for (src, dest) in copy_plan {
        let from = template_root.join(src);
        let to = project_root.join(dest);

        debug!("Copying: {} → {}", from.display(), to.display());

        if from.extension().map_or(false, |e| e == "ico") {
            let bytes = fs::read(&from)
                .with_context(|| format!("Failed to read binary file: {}", from.display()))?;
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create dir: {}", parent.display()))?;
            }
            fs::write(&to, bytes)
                .with_context(|| format!("Failed to write binary file: {}", to.display()))?;
        } else {
            template::copy_template_file(&from, &to, name)?;
        }

        info!("✅ Created {}", to.display());
    }

    info!("🎉 Project '{}' scaffolded successfully!", name);
    Ok(())
}
