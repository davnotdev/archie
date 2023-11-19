use super::*;

pub fn go_check() -> Result<bool> {
    Ok(Command::new("go").output().is_ok())
}

pub fn go_visitor(dir: &Path) -> Result<()> {
    for subdir in fs::read_dir(dir)? {
        let subdir = subdir?;
        if subdir.file_type()?.is_dir() && subdir.file_name() == "go.mod" {
            Command::new("go").arg("clean").current_dir(dir).output()?;
            break;
        }
    }
    Ok(())
}
