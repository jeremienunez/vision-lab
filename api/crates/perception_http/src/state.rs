use std::sync::Arc;

use perception_app::{
    AnnotationRepository, DatasetRepository, DatasetVersionRepository, SampleRepository,
    SampleStorage,
};

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

#[derive(Clone)]
pub struct AnnotationHttpState {
    dataset_repository: Arc<dyn DatasetRepository>,
    sample_repository: Arc<dyn SampleRepository>,
    annotation_repository: Arc<dyn AnnotationRepository>,
}

impl AnnotationHttpState {
    pub fn new(
        dataset_repository: Arc<dyn DatasetRepository>,
        sample_repository: Arc<dyn SampleRepository>,
        annotation_repository: Arc<dyn AnnotationRepository>,
    ) -> Self {
        Self {
            dataset_repository,
            sample_repository,
            annotation_repository,
        }
    }

    pub fn dataset_repository(&self) -> &dyn DatasetRepository {
        self.dataset_repository.as_ref()
    }

    pub fn sample_repository(&self) -> &dyn SampleRepository {
        self.sample_repository.as_ref()
    }

    pub fn annotation_repository(&self) -> &dyn AnnotationRepository {
        self.annotation_repository.as_ref()
    }
}

#[derive(Clone)]
pub struct DatasetStatsHttpState {
    dataset_repository: Arc<dyn DatasetRepository>,
    sample_repository: Arc<dyn SampleRepository>,
    annotation_repository: Arc<dyn AnnotationRepository>,
}

impl DatasetStatsHttpState {
    pub fn new(
        dataset_repository: Arc<dyn DatasetRepository>,
        sample_repository: Arc<dyn SampleRepository>,
        annotation_repository: Arc<dyn AnnotationRepository>,
    ) -> Self {
        Self {
            dataset_repository,
            sample_repository,
            annotation_repository,
        }
    }

    pub fn dataset_repository(&self) -> &dyn DatasetRepository {
        self.dataset_repository.as_ref()
    }

    pub fn sample_repository(&self) -> &dyn SampleRepository {
        self.sample_repository.as_ref()
    }

    pub fn annotation_repository(&self) -> &dyn AnnotationRepository {
        self.annotation_repository.as_ref()
    }
}

#[derive(Clone)]
pub struct DatasetVersionHttpState {
    dataset_repository: Arc<dyn DatasetRepository>,
    sample_repository: Arc<dyn SampleRepository>,
    annotation_repository: Arc<dyn AnnotationRepository>,
    dataset_version_repository: Arc<dyn DatasetVersionRepository>,
}

impl DatasetVersionHttpState {
    pub fn new(
        dataset_repository: Arc<dyn DatasetRepository>,
        sample_repository: Arc<dyn SampleRepository>,
        annotation_repository: Arc<dyn AnnotationRepository>,
        dataset_version_repository: Arc<dyn DatasetVersionRepository>,
    ) -> Self {
        Self {
            dataset_repository,
            sample_repository,
            annotation_repository,
            dataset_version_repository,
        }
    }

    pub fn dataset_repository(&self) -> &dyn DatasetRepository {
        self.dataset_repository.as_ref()
    }

    pub fn sample_repository(&self) -> &dyn SampleRepository {
        self.sample_repository.as_ref()
    }

    pub fn annotation_repository(&self) -> &dyn AnnotationRepository {
        self.annotation_repository.as_ref()
    }

    pub fn dataset_version_repository(&self) -> &dyn DatasetVersionRepository {
        self.dataset_version_repository.as_ref()
    }
}
