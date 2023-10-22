use super::*;
use std::{fs, path::Path, process::Command};

mod cargo;
mod make;
mod ninja;
mod npm;

type Visitor = fn(dir: &Path) -> Result<()>;
type Check = fn() -> Result<bool>;

pub fn brush_directory<P: AsRef<Path>>(target: P) -> Result<()> {
    fn recur(target: &Path, brush_name: &str, visit: &Visitor) -> Result<()> {
        if let Err(e) = visit(target) {
            println!(
                "Warning: failed to run {} on {}: {:?}",
                brush_name,
                target.display(),
                e
            );
        }
        for dir in fs::read_dir(target)? {
            let dir = dir?;
            if dir.file_type()?.is_dir() {
                recur(&dir.path(), brush_name, visit)?;
            }
        }
        Ok(())
    }

    for (brush_name, check, visit) in brushes() {
        if check()? {
            recur(target.as_ref(), brush_name, visit)?;
        };
    }

    Ok(())
}

fn brushes() -> &'static [(&'static str, Check, Visitor)] {
    &[
        ("cargo", cargo::cargo_check, cargo::cargo_visitor),
        ("npm", npm::npm_check, npm::npm_visitor),
        ("make", make::make_check, make::make_visitor),
        ("ninja", ninja::ninja_check, ninja::ninja_visitor),
    ]
}
