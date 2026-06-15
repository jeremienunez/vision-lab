use std::collections::BTreeMap;

use perception_domain::DatasetId;

use crate::{
    AnnotationRepository, DatasetRepository, DatasetStats, SampleRepository, UseCaseError,
};

pub struct DatasetStatsUseCase<'repository> {
    dataset_repository: &'repository dyn DatasetRepository,
    sample_repository: &'repository dyn SampleRepository,
    annotation_repository: &'repository dyn AnnotationRepository,
}

impl<'repository> DatasetStatsUseCase<'repository> {
    pub fn new(
        dataset_repository: &'repository dyn DatasetRepository,
        sample_repository: &'repository dyn SampleRepository,
        annotation_repository: &'repository dyn AnnotationRepository,
    ) -> Self {
        Self {
            dataset_repository,
            sample_repository,
            annotation_repository,
        }
    }

    pub async fn execute(&self, dataset_id: DatasetId) -> Result<DatasetStats, UseCaseError> {
        if self.dataset_repository.get(dataset_id).await?.is_none() {
            return Err(UseCaseError::NotFound("dataset not found"));
        }

        let samples = self.sample_repository.list_by_dataset(dataset_id).await?;
        let annotations = self
            .annotation_repository
            .list_by_dataset(dataset_id)
            .await?;
        let mut annotations_by_class = BTreeMap::new();

        for annotation in &annotations {
            *annotations_by_class
                .entry(annotation.class_name.clone())
                .or_insert(0) += 1;
        }

        Ok(DatasetStats {
            dataset_id,
            sample_count: samples.len() as u64,
            annotation_count: annotations.len() as u64,
            annotations_by_class,
        })
    }
}
