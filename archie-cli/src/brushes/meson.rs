use super::*;

pub fn meson_check() -> Result<bool> {
    Ok(Command::new("meson").output().is_ok())
}

pub fn meson_visitor(dir: &Path) -> Result<()> {
    //  Since there doesn't seem to be a reliable way to detect meson's build dir, my current
    //  method is to find the `meson-info` and `meson-private` dirs.
    let mut has_meson_info = false;
    let mut has_meson_private = false;

    for subdir in fs::read_dir(dir)? {
        let subdir = subdir?;
        if subdir.file_type()?.is_dir() && subdir.file_name() == "meson-info" {
            has_meson_info = true;
        }
        if subdir.file_type()?.is_dir() && subdir.file_name() == "meson-private" {
            has_meson_private = true;
        }
    }
    if has_meson_info && has_meson_private {
        Command::new("meson")
            .arg("compile")
            .arg("--clean")
            .current_dir(dir)
            .output()?;
    }
    Ok(())
}
