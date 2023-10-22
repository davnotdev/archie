use super::*;

pub fn ninja_check() -> Result<bool> {
    Ok(Command::new("ninja").output().is_ok())
}

pub fn ninja_visitor(dir: &Path) -> Result<()> {
    for dir in fs::read_dir(dir)? {
        let dir = dir?;
        if dir.file_type()?.is_file() && dir.file_name() == "build.ninja" {
            let mut clean_dir = dir.path();
            clean_dir.pop();
            Command::new("ninja")
                .arg("clean")
                .arg("-C")
                .arg(clean_dir)
                .output()?;
        }
    }
    Ok(())
}
