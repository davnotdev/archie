use super::*;

pub fn cargo_check() -> Result<bool> {
    Ok(Command::new("cargo").output().is_ok())
}

pub fn cargo_visitor(dir: &Path) -> Result<()> {
    for dir in fs::read_dir(dir)? {
        let dir = dir?;
        if dir.file_type()?.is_file() && dir.file_name() == "Cargo.lock" {
            let mut manifest = dir.path();
            manifest.pop();
            manifest.push("Cargo.toml");
            Command::new("cargo")
                .arg("clean")
                .arg("--manifest-path")
                .arg(manifest)
                .output()?;
        }
    }
    Ok(())
}
