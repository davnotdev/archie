use super::*;
use std::{fs, path::Path, process::Command};

mod cargo;
mod go;
mod gradle;
mod luarocks;
mod make;
mod meson;
mod ninja;
mod npm;
mod nuget;
mod pod;
mod swift;

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
        ("go", go::go_check, go::go_visitor),
        ("gradle", gradle::gradle_check, gradle::gradle_visitor),
        (
            "luarocks",
            luarocks::luarocks_check,
            luarocks::luarocks_visitor,
        ),
        ("make", make::make_check, make::make_visitor),
        ("meson", meson::meson_check, meson::meson_visitor),
        ("ninja", ninja::ninja_check, ninja::ninja_visitor),
        ("npm", npm::npm_check, npm::npm_visitor),
        ("nuget", nuget::nuget_check, nuget::nuget_visitor),
        ("pod", pod::pod_check, pod::pod_visitor),
        ("swift", swift::swift_check, swift::swift_visitor),
    ]
}
