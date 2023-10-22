use super::*;
use b64::{FromBase64, ToBase64};
use std::{fs, path::PathBuf};

pub struct Archive {
    config: Config,
    archived_directories: Vec<String>,
}

impl Archive {
    pub fn new(config: Config) -> Result<Self> {
        let _ = fs::create_dir_all(&config.archive_location);

        let mut archived_directories = vec![];
        for file in fs::read_dir(&config.archive_location)? {
            let file = file?;
            if file.file_type()?.is_file() {
                archived_directories.push(file.file_name().to_str().unwrap().to_owned());
            }
        }

        Ok(Self {
            config,
            archived_directories,
        })
    }

    pub fn push(&self, target: &str) -> Result<()> {
        let b64_target = fs::canonicalize(target)?
            .to_str()
            .unwrap()
            .as_bytes()
            .to_base64(b64::URL_SAFE);
        if self
            .archived_directories
            .iter()
            .any(|file| *file == b64_target)
        {
            anyhow::bail!("Duplicate push!");
        }

        let archive = create_tar_archive(target)?;
        let compressed = Compression::compress(&self.config, &archive)?;
        let mut write_path = PathBuf::from(&self.config.archive_location);
        write_path.push(b64_target);
        fs::write(write_path, compressed)?;
        fs::remove_dir_all(target)?;
        fs::create_dir(target)?;

        Ok(())
    }

    pub fn pull(&self, target: &str) -> Result<()> {
        let b64_target = fs::canonicalize(target)?
            .to_str()
            .unwrap()
            .as_bytes()
            .to_base64(b64::URL_SAFE);

        if self
            .archived_directories
            .iter()
            .any(|target| **target == b64_target)
        {
            let mut tar_path = PathBuf::from(&self.config.archive_location);
            tar_path.push(b64_target);

            let compressed = fs::read(&tar_path)?;
            let archive = Compression::decompress(&self.config, &compressed)?;
            extract_tar_archive(&archive, target)?;
            fs::remove_file(tar_path)?;
            Ok(())
        } else {
            anyhow::bail!("No such pull!");
        }
    }

    pub fn list(&self) -> Result<()> {
        for dir in self.archived_directories.iter() {
            println!("{} {}", String::from_utf8(dir.from_base64()?)?, dir);
        }
        Ok(())
    }
}
