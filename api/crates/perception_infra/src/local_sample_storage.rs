use std::{
    env,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use perception_app::{SampleStorage, SampleStorageCommand, StoredSample, UseCaseError};
use sha2::{Digest, Sha256};
use tokio::fs;

#[derive(Debug, Clone)]
pub struct LocalSampleStorage {
    root: PathBuf,
}

impl LocalSampleStorage {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }
}

#[async_trait]
impl SampleStorage for LocalSampleStorage {
    async fn store(&self, command: SampleStorageCommand) -> Result<StoredSample, UseCaseError> {
        let filename = safe_filename(&command.filename)?;
        let directory = self
            .root
            .join("datasets")
            .join(command.dataset_id.to_string())
            .join("samples")
            .join(command.sample_id.to_string());
        let path = directory.join(filename);

        fs::create_dir_all(&directory)
            .await
            .map_err(|_| UseCaseError::Repository("sample storage directory creation failed"))?;
        fs::write(&path, &command.bytes)
            .await
            .map_err(|_| UseCaseError::Repository("sample storage write failed"))?;

        let absolute_path = if path.is_absolute() {
            path
        } else {
            env::current_dir()
                .map_err(|_| UseCaseError::Repository("current directory unavailable"))?
                .join(path)
        };
        let checksum = Sha256::digest(&command.bytes);

        Ok(StoredSample {
            storage_uri: format!("file://{}", absolute_path.display()),
            size_bytes: command.bytes.len() as u64,
            checksum: format!("sha256:{}", hex::encode(checksum)),
        })
    }
}

fn safe_filename(filename: &str) -> Result<&str, UseCaseError> {
    Path::new(filename)
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .ok_or(UseCaseError::Validation("sample filename is required"))
}
