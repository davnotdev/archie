use super::*;

pub fn make_check() -> Result<bool> {
    Ok(Command::new("make").output().is_ok())
}

pub fn make_visitor(dir: &Path) -> Result<()> {
    for dir in fs::read_dir(dir)? {
        let dir = dir?;
        if dir.file_type()?.is_file() && dir.file_name() == "build.ninja" {
            let mut clean_dir = dir.path();
            clean_dir.pop();
            Command::new("make")
                .arg("clean")
                .arg("-C")
                .arg(clean_dir)
                .output()?;
        }
    }
    Ok(())
}
