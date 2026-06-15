use std::collections::BTreeMap;

use perception_domain::{DatasetId, ImageDimensions, SampleId};

use crate::{
    SampleDraft, SampleRepository, SampleStorage, SampleStorageCommand, UseCaseError,
    ports::DatasetRepository,
};

const MAX_SAMPLE_BYTES: usize = 25 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadSampleCommand {
    pub dataset_id: DatasetId,
    pub filename: String,
    pub mime_type: String,
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

pub struct UploadSampleUseCase<'repository> {
    dataset_repository: &'repository dyn DatasetRepository,
    sample_repository: &'repository dyn SampleRepository,
    sample_storage: &'repository dyn SampleStorage,
}

impl<'repository> UploadSampleUseCase<'repository> {
    pub fn new(
        dataset_repository: &'repository dyn DatasetRepository,
        sample_repository: &'repository dyn SampleRepository,
        sample_storage: &'repository dyn SampleStorage,
    ) -> Self {
        Self {
            dataset_repository,
            sample_repository,
            sample_storage,
        }
    }

    pub async fn execute(&self, command: UploadSampleCommand) -> Result<SampleDraft, UseCaseError> {
        if self
            .dataset_repository
            .get(command.dataset_id)
            .await?
            .is_none()
        {
            return Err(UseCaseError::NotFound("dataset not found"));
        }

        let filename = validate_filename(&command.filename)?;
        let mime_type = validate_mime_type(&command.mime_type)?;
        let dimensions = ImageDimensions::new(command.width, command.height)
            .map_err(|_| UseCaseError::Validation("image dimensions must be positive"))?;

        if command.bytes.is_empty() {
            return Err(UseCaseError::Validation("sample file is required"));
        }

        if command.bytes.len() > MAX_SAMPLE_BYTES {
            return Err(UseCaseError::Validation("sample file too large"));
        }

        let sample_id = SampleId::new();
        let stored = self
            .sample_storage
            .store(SampleStorageCommand {
                dataset_id: command.dataset_id,
                sample_id,
                filename: filename.clone(),
                mime_type: mime_type.clone(),
                bytes: command.bytes,
            })
            .await?;

        self.sample_repository
            .create(SampleDraft {
                id: sample_id,
                dataset_id: command.dataset_id,
                storage_uri: stored.storage_uri,
                filename,
                mime_type,
                width: dimensions.width,
                height: dimensions.height,
                size_bytes: stored.size_bytes,
                checksum: stored.checksum,
                source: "upload".to_owned(),
                metadata: BTreeMap::new(),
            })
            .await
    }
}

fn validate_filename(filename: &str) -> Result<String, UseCaseError> {
    let trimmed = filename.trim();

    if trimmed.is_empty() || trimmed.contains('/') || trimmed.contains('\\') {
        return Err(UseCaseError::Validation("sample filename is required"));
    }

    Ok(trimmed.to_owned())
}

fn validate_mime_type(mime_type: &str) -> Result<String, UseCaseError> {
    let normalized = mime_type.trim().to_ascii_lowercase();
    let supported = matches!(
        normalized.as_str(),
        "image/jpeg" | "image/jpg" | "image/png" | "image/webp"
    );

    if supported {
        Ok(normalized)
    } else {
        Err(UseCaseError::Validation("unsupported image mime type"))
    }
}
