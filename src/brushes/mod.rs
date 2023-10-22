use super::*;
use std::{fs, path::Path, process::Command};

mod cargo;
mod npm;

type Visitor = fn(dir: &Path);
type Check = fn();

pub fn brush_directory() {
    todo!()
}
