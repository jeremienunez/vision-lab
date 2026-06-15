use std::sync::Arc;

use perception_app::{
    AnnotationRepository, DatasetRepository, DatasetVersionRepository, SampleRepository,
    SampleStorage, TrainingJobQueue, TrainingJobRepository,
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

#[derive(Clone)]
pub struct TrainingJobHttpState {
    dataset_version_repository: Arc<dyn DatasetVersionRepository>,
    training_job_repository: Arc<dyn TrainingJobRepository>,
    training_job_queue: Arc<dyn TrainingJobQueue>,
}

impl TrainingJobHttpState {
    pub fn new(
        dataset_version_repository: Arc<dyn DatasetVersionRepository>,
        training_job_repository: Arc<dyn TrainingJobRepository>,
        training_job_queue: Arc<dyn TrainingJobQueue>,
    ) -> Self {
        Self {
            dataset_version_repository,
            training_job_repository,
            training_job_queue,
        }
    }

    pub fn dataset_version_repository(&self) -> &dyn DatasetVersionRepository {
        self.dataset_version_repository.as_ref()
    }

    pub fn training_job_repository(&self) -> &dyn TrainingJobRepository {
        self.training_job_repository.as_ref()
    }

    pub fn training_job_queue(&self) -> &dyn TrainingJobQueue {
        self.training_job_queue.as_ref()
    }
}
