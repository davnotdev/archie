use super::*;

pub fn ninja_check() -> Result<bool> {
    Ok(Command::new("ninja").output().is_ok())
}

pub fn ninja_visitor(dir: &Path) -> Result<()> {
    for subdir in fs::read_dir(dir)? {
        let subdir = subdir?;
        if subdir.file_type()?.is_file() && subdir.file_name() == "build.ninja" {
            Command::new("ninja")
                .arg("clean")
                .arg("-C")
                .arg(dir)
                .output()?;
            break;
        }
    }
    Ok(())
}
