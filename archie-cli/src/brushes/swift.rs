use super::*;

pub fn swift_check() -> Result<bool> {
    Ok(Command::new("swift").output().is_ok())
}

pub fn swift_visitor(dir: &Path) -> Result<()> {
    for subdir in fs::read_dir(dir)? {
        let subdir = subdir?;
        if subdir.file_type()?.is_file() && subdir.file_name() == "Package.swift" {
            Command::new("swift")
                .arg("package")
                .arg("clean")
                .arg(dir)
                .output()?;
            break;
        }
    }
    Ok(())
}
