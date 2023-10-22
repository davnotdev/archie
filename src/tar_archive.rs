use super::*;
use std::{fs, path::Path};
use tar::{Archive, Builder};

pub fn create_tar_archive<P: AsRef<Path>>(target_dir: P) -> Result<Vec<u8>> {
    let mut builder = Builder::new(vec![]);

    fn add_directory_contents(
        builder: &mut Builder<Vec<u8>>,
        base: &Path,
        path: &Path,
    ) -> Result<()> {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let rel_path = path.clone();
            let rel_path = rel_path.strip_prefix(base).unwrap();

            if path.is_dir() {
                builder.append_dir(rel_path, path.clone())?;
                add_directory_contents(builder, base, path.as_path())?;
            } else {
                builder.append_file(rel_path, &mut fs::File::open(path)?)?;
            }
        }
        Ok(())
    }

    add_directory_contents(&mut builder, target_dir.as_ref(), target_dir.as_ref())?;
    builder.finish()?;
    Ok(builder.into_inner()?)
}

pub fn extract_tar_archive<P: AsRef<Path>>(archive: &[u8], output_dir: P) -> Result<()> {
    let mut archive = Archive::new(archive);
    archive.unpack(output_dir)?;
    Ok(())
}
