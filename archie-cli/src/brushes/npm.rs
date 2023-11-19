use super::*;

pub fn npm_check() -> Result<bool> {
    Ok(true)
}

pub fn npm_visitor(dir: &Path) -> Result<()> {
    for subdir in fs::read_dir(dir)? {
        let subdir = subdir?;
        if subdir.file_type()?.is_dir() && subdir.file_name() == "node_modules" {
            fs::remove_dir_all(subdir.path())?;
            break;
        }
    }
    Ok(())
}
