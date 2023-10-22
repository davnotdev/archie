use super::*;

pub fn npm_check() -> Result<bool> {
    Ok(Command::new("npm").output().is_ok())
}

pub fn npm_visitor(dir: &Path) -> Result<()> {
    for dir in fs::read_dir(dir)? {
        let dir = dir?;
        if dir.file_type()?.is_dir() && dir.file_name() == "node_modules" {
            fs::remove_dir_all(dir.path())?;
        }
    }
    Ok(())
}
