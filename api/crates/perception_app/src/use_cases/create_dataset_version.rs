use std::collections::BTreeMap;

use perception_domain::{DatasetId, DatasetVersionId};

use crate::{
    AnnotationRepository, DatasetRepository, DatasetVersionDraft, DatasetVersionRepository,
    SampleRepository, UseCaseError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateDatasetVersionCommand {
    pub dataset_id: DatasetId,
    pub version_name: String,
    pub split_config: BTreeMap<String, String>,
    pub created_by: String,
}

pub struct CreateDatasetVersionUseCase<'repository> {
    dataset_repository: &'repository dyn DatasetRepository,
    sample_repository: &'repository dyn SampleRepository,
    annotation_repository: &'repository dyn AnnotationRepository,
    dataset_version_repository: &'repository dyn DatasetVersionRepository,
}

impl<'repository> CreateDatasetVersionUseCase<'repository> {
    pub fn new(
        dataset_repository: &'repository dyn DatasetRepository,
        sample_repository: &'repository dyn SampleRepository,
        annotation_repository: &'repository dyn AnnotationRepository,
        dataset_version_repository: &'repository dyn DatasetVersionRepository,
    ) -> Self {
        Self {
            dataset_repository,
            sample_repository,
            annotation_repository,
            dataset_version_repository,
        }
    }

    pub async fn execute(
        &self,
        command: CreateDatasetVersionCommand,
    ) -> Result<DatasetVersionDraft, UseCaseError> {
        let dataset = self
            .dataset_repository
            .get(command.dataset_id)
            .await?
            .ok_or(UseCaseError::NotFound("dataset not found"))?;

        if command.version_name.trim().is_empty() {
            return Err(UseCaseError::Validation("dataset version name is required"));
        }

        if dataset.classes.is_empty() {
            return Err(UseCaseError::Validation(
                "dataset version requires at least one class",
            ));
        }

        let samples = self
            .sample_repository
            .list_by_dataset(command.dataset_id)
            .await?;

        if samples.is_empty() {
            return Err(UseCaseError::Validation(
                "dataset version requires at least one sample",
            ));
        }

        let split_config = validate_split_config(command.split_config)?;

        let annotations = self
            .annotation_repository
            .list_by_dataset(command.dataset_id)
            .await?;

        self.dataset_version_repository
            .create(DatasetVersionDraft {
                id: DatasetVersionId::new(),
                dataset_id: command.dataset_id,
                version_name: command.version_name.trim().to_owned(),
                sample_count: samples.len() as u64,
                annotation_count: annotations.len() as u64,
                classes_snapshot: dataset.classes,
                split_config,
                created_by: command.created_by.trim().to_owned(),
            })
            .await
    }
}

fn validate_split_config(
    split_config: BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>, UseCaseError> {
    if split_config.is_empty() {
        return Ok(split_config);
    }

    let mut normalized = BTreeMap::new();
    let mut total = 0_u16;

    for split_name in ["train", "validation", "test"] {
        let percentage = split_config
            .get(split_name)
            .ok_or(UseCaseError::Validation(
                "dataset split requires train validation and test",
            ))?
            .parse::<u16>()
            .map_err(|_| UseCaseError::Validation("dataset split values must be percentages"))?;

        if percentage == 0 || percentage > 100 {
            return Err(UseCaseError::Validation(
                "dataset split values must be between 1 and 100",
            ));
        }

        total += percentage;
        normalized.insert(split_name.to_owned(), percentage.to_string());
    }

    if total != 100 {
        return Err(UseCaseError::Validation("dataset split must sum to 100"));
    }

    Ok(normalized)
}
