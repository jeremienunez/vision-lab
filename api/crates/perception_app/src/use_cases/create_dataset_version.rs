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
                split_config: BTreeMap::new(),
                created_by: command.created_by.trim().to_owned(),
            })
            .await
    }
}
