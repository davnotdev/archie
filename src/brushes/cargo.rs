use super::*;

pub fn cargo_check() -> Result<bool> {
    Ok(Command::new("cargo").output().is_ok())
}

pub fn cargo_visitor(dir: &Path) -> Result<()> {
    for subdir in fs::read_dir(dir)? {
        let subdir = subdir?;
        if subdir.file_type()?.is_file() && subdir.file_name() == "Cargo.lock" {
            let mut manifest = subdir.path();
            manifest.pop();
            manifest.push("Cargo.toml");
            Command::new("cargo")
                .arg("clean")
                .arg("--manifest-path")
                .arg(manifest)
                .output()?;
            break;
        }
    }
    Ok(())
}
