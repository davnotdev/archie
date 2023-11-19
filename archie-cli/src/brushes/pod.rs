use super::*;

pub fn pod_check() -> Result<bool> {
    Ok(Command::new("pod").output().is_ok())
}

pub fn pod_visitor(dir: &Path) -> Result<()> {
    for subdir in fs::read_dir(dir)? {
        let subdir = subdir?;
        if subdir.file_type()?.is_file() && subdir.file_name() == "" {
            Command::new("pod")
                .arg("cache")
                .arg("clean")
                .arg("--all")
                .arg(dir)
                .output()?;
            break;
        }
    }
    Ok(())
}
