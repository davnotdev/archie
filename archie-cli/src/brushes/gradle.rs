use super::*;

pub fn gradle_check() -> Result<bool> {
    Ok(Command::new("gradle").output().is_ok())
}

pub fn gradle_visitor(dir: &Path) -> Result<()> {
    for subdir in fs::read_dir(dir)? {
        let subdir = subdir?;
        if subdir.file_type()?.is_file() && subdir.file_name() == "gradlew" {
            Command::new(subdir.path().to_str().unwrap())
                .arg("clean")
                .current_dir(dir)
                .output()?;
            break;
        }
    }
    Ok(())
}
