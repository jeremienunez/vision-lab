use std::sync::Arc;

use perception_app::{DatasetRepository, SampleRepository, SampleStorage};

#[derive(Clone)]
pub struct HttpState {
    dataset_repository: Arc<dyn DatasetRepository>,
}

impl HttpState {
    pub fn new(dataset_repository: Arc<dyn DatasetRepository>) -> Self {
        Self { dataset_repository }
    }

    pub fn dataset_repository(&self) -> &dyn DatasetRepository {
        self.dataset_repository.as_ref()
    }
}

#[derive(Clone)]
pub struct SampleHttpState {
    dataset_repository: Arc<dyn DatasetRepository>,
    sample_repository: Arc<dyn SampleRepository>,
    sample_storage: Arc<dyn SampleStorage>,
}

impl SampleHttpState {
    pub fn new(
        dataset_repository: Arc<dyn DatasetRepository>,
        sample_repository: Arc<dyn SampleRepository>,
        sample_storage: Arc<dyn SampleStorage>,
    ) -> Self {
        Self {
            dataset_repository,
            sample_repository,
            sample_storage,
        }
    }

    pub fn dataset_repository(&self) -> &dyn DatasetRepository {
        self.dataset_repository.as_ref()
    }

    pub fn sample_repository(&self) -> &dyn SampleRepository {
        self.sample_repository.as_ref()
    }

    pub fn sample_storage(&self) -> &dyn SampleStorage {
        self.sample_storage.as_ref()
    }
}
