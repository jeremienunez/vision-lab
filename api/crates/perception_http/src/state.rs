use std::sync::Arc;

use perception_app::{
    AnnotationRepository, DatasetRepository, DatasetVersionRepository, InferenceEngine,
    InferenceRunRepository, ModelExportRepository, ModelRepository, OverlayRenderer,
    SampleRepository, SampleStorage, TrainingJobQueue, TrainingJobRepository,
    TrainingMetricRepository,
};

#[derive(Clone)]
pub struct HttpState {
    dataset_repository: Arc<dyn DatasetRepository>,
}

#[derive(Clone)]
pub struct ModelHttpState {
    model_repository: Arc<dyn ModelRepository>,
    model_export_repository: Arc<dyn ModelExportRepository>,
    inference_run_repository: Arc<dyn InferenceRunRepository>,
    overlay_renderer: Arc<dyn OverlayRenderer>,
    inference_engine: Arc<dyn InferenceEngine>,
}

#[derive(Clone)]
pub struct ModelRegistrationHttpState {
    training_job_repository: Arc<dyn TrainingJobRepository>,
    model_repository: Arc<dyn ModelRepository>,
}

impl ModelRegistrationHttpState {
    pub fn new(
        training_job_repository: Arc<dyn TrainingJobRepository>,
        model_repository: Arc<dyn ModelRepository>,
    ) -> Self {
        Self {
            training_job_repository,
            model_repository,
        }
    }

    pub fn training_job_repository(&self) -> &dyn TrainingJobRepository {
        self.training_job_repository.as_ref()
    }

    pub fn model_repository(&self) -> &dyn ModelRepository {
        self.model_repository.as_ref()
    }
}

impl ModelHttpState {
    pub fn new(
        model_repository: Arc<dyn ModelRepository>,
        model_export_repository: Arc<dyn ModelExportRepository>,
        inference_run_repository: Arc<dyn InferenceRunRepository>,
        overlay_renderer: Arc<dyn OverlayRenderer>,
        inference_engine: Arc<dyn InferenceEngine>,
    ) -> Self {
        Self {
            model_repository,
            model_export_repository,
            inference_run_repository,
            overlay_renderer,
            inference_engine,
        }
    }

    pub fn model_repository(&self) -> &dyn ModelRepository {
        self.model_repository.as_ref()
    }

    pub fn model_export_repository(&self) -> &dyn ModelExportRepository {
        self.model_export_repository.as_ref()
    }

    pub fn inference_run_repository(&self) -> &dyn InferenceRunRepository {
        self.inference_run_repository.as_ref()
    }

    pub fn overlay_renderer(&self) -> &dyn OverlayRenderer {
        self.overlay_renderer.as_ref()
    }

    pub fn inference_engine(&self) -> &dyn InferenceEngine {
        self.inference_engine.as_ref()
    }
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
    training_metric_repository: Arc<dyn TrainingMetricRepository>,
}

impl TrainingJobHttpState {
    pub fn new(
        dataset_version_repository: Arc<dyn DatasetVersionRepository>,
        training_job_repository: Arc<dyn TrainingJobRepository>,
        training_job_queue: Arc<dyn TrainingJobQueue>,
        training_metric_repository: Arc<dyn TrainingMetricRepository>,
    ) -> Self {
        Self {
            dataset_version_repository,
            training_job_repository,
            training_job_queue,
            training_metric_repository,
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

    pub fn training_metric_repository(&self) -> &dyn TrainingMetricRepository {
        self.training_metric_repository.as_ref()
    }
}
