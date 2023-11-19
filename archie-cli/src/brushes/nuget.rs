use super::*;

pub fn nuget_check() -> Result<bool> {
    Ok(Command::new("nuget").output().is_ok())
}

pub fn nuget_visitor(dir: &Path) -> Result<()> {
    //  I know for a fact that there are a trillion more ways to have a nuget project.
    //  Microsoft never fails to confuse me.
    for subdir in fs::read_dir(dir)? {
        let subdir = subdir?;
        let file_name = subdir.file_name().to_str().unwrap().to_lowercase();
        if subdir.file_type()?.is_file()
            && (file_name == "nuget.config" || file_name.contains(".nuspec"))
        {
            Command::new("nuget")
                .arg("locals")
                .arg("all")
                .arg("-clear")
                .current_dir(dir)
                .output()?;
            break;
        }
    }
    Ok(())
}
