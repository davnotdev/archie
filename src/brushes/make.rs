use super::*;

pub fn make_check() -> Result<bool> {
    Ok(Command::new("make").output().is_ok())
}

pub fn make_visitor(dir: &Path) -> Result<()> {
    for subdir in fs::read_dir(dir)? {
        let subdir = subdir?;
        if subdir.file_type()?.is_file() && subdir.file_name() == "build.ninja" {
            Command::new("make")
                .arg("clean")
                .arg("-C")
                .arg(dir)
                .output()?;
            break;
        }
    }
    Ok(())
}
