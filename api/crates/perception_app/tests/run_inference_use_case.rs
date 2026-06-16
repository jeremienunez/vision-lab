use std::{collections::BTreeMap, sync::Mutex};

use async_trait::async_trait;
use perception_app::{
    DetectionDraft, InferenceEngine, InferenceRequest, InferenceResult, InferenceRunDraft,
    InferenceRunRepository, ModelDraft, ModelRepository, RunInferenceCommand, RunInferenceUseCase,
    UseCaseError,
};
use perception_domain::{
    DatasetVersionId, InferenceRunId, ModelId, ModelStatus, NormalizedBbox, TrainingJobId,
};

#[derive(Default)]
struct InMemoryModelRepository {
    models: Mutex<Vec<ModelDraft>>,
}

#[async_trait]
impl ModelRepository for InMemoryModelRepository {
    async fn create(&self, model: ModelDraft) -> Result<ModelDraft, UseCaseError> {
        self.models
            .lock()
            .expect("repository mutex is available")
            .push(model.clone());
        Ok(model)
    }

    async fn list(&self) -> Result<Vec<ModelDraft>, UseCaseError> {
        Ok(self
            .models
            .lock()
            .expect("repository mutex is available")
            .clone())
    }

    async fn get(&self, model_id: ModelId) -> Result<Option<ModelDraft>, UseCaseError> {
        Ok(self
            .models
            .lock()
            .expect("repository mutex is available")
            .iter()
            .find(|model| model.id == model_id)
            .cloned())
    }

    async fn update(&self, model: ModelDraft) -> Result<ModelDraft, UseCaseError> {
        let mut models = self.models.lock().expect("repository mutex is available");
        let stored = models
            .iter_mut()
            .find(|stored_model| stored_model.id == model.id)
            .ok_or(UseCaseError::NotFound("model not found"))?;
        *stored = model.clone();
        Ok(model)
    }
}

#[derive(Default)]
struct InMemoryInferenceRunRepository {
    runs: Mutex<Vec<InferenceRunDraft>>,
}

#[async_trait]
impl InferenceRunRepository for InMemoryInferenceRunRepository {
    async fn create(&self, run: InferenceRunDraft) -> Result<InferenceRunDraft, UseCaseError> {
        self.runs
            .lock()
            .expect("repository mutex is available")
            .push(run.clone());
        Ok(run)
    }

    async fn get(&self, run_id: InferenceRunId) -> Result<Option<InferenceRunDraft>, UseCaseError> {
        Ok(self
            .runs
            .lock()
            .expect("repository mutex is available")
            .iter()
            .find(|run| run.id == run_id)
            .cloned())
    }
}

struct TestInferenceEngine;

#[async_trait]
impl InferenceEngine for TestInferenceEngine {
    async fn infer(&self, request: InferenceRequest) -> Result<InferenceResult, UseCaseError> {
        Ok(InferenceResult {
            run_id: InferenceRunId::new(),
            model_id: request.model.id,
            latency_ms: 7,
            detections: vec![
                DetectionDraft {
                    class_id: 0,
                    class_name: "cup".to_owned(),
                    confidence: 0.95,
                    bbox: NormalizedBbox::new(0.1, 0.2, 0.3, 0.4).expect("bbox is valid"),
                    distance_m: None,
                },
                DetectionDraft {
                    class_id: 1,
                    class_name: "book".to_owned(),
                    confidence: 0.42,
                    bbox: NormalizedBbox::new(0.2, 0.3, 0.2, 0.2).expect("bbox is valid"),
                    distance_m: None,
                },
            ],
        })
    }
}

fn model_fixture(status: ModelStatus) -> ModelDraft {
    ModelDraft {
        id: ModelId::new(),
        name: "desk-objects".to_owned(),
        version: "v1".to_owned(),
        training_job_id: TrainingJobId::new(),
        dataset_version_id: DatasetVersionId::new(),
        model_family: "tiny_torch".to_owned(),
        artifact_uri: "file:///tmp/model.pt".to_owned(),
        metrics_summary: BTreeMap::new(),
        status,
    }
}

#[tokio::test]
async fn run_inference_returns_detections_filtered_by_threshold() {
    let models = InMemoryModelRepository::default();
    let runs = InMemoryInferenceRunRepository::default();
    let model = models
        .create(model_fixture(ModelStatus::Candidate))
        .await
        .expect("model is stored");

    let result = RunInferenceUseCase::new(&models, &runs, &TestInferenceEngine)
        .execute(RunInferenceCommand {
            model_id: model.id,
            filename: "cup.jpg".to_owned(),
            mime_type: "image/jpeg".to_owned(),
            image_bytes: vec![1, 2, 3],
            confidence_threshold: 0.90,
        })
        .await
        .expect("inference succeeds");

    assert_eq!(result.model_id, model.id);
    assert_eq!(result.latency_ms, 7);
    assert_eq!(result.detections.len(), 1);
    assert_eq!(result.detections[0].class_name, "cup");
    assert!(result.detections[0].confidence >= 0.90);

    let stored = runs
        .get(result.run_id)
        .await
        .expect("run lookup succeeds")
        .expect("run is stored");

    assert_eq!(stored.id, result.run_id);
    assert_eq!(stored.model_id, model.id);
    assert_eq!(stored.filename, "cup.jpg");
    assert_eq!(stored.mime_type, "image/jpeg");
    assert_eq!(stored.detections.len(), 1);
    assert_eq!(stored.detections[0].class_name, "cup");
}

#[tokio::test]
async fn run_inference_rejects_missing_or_archived_model() {
    let models = InMemoryModelRepository::default();
    let runs = InMemoryInferenceRunRepository::default();

    let missing = RunInferenceUseCase::new(&models, &runs, &TestInferenceEngine)
        .execute(RunInferenceCommand {
            model_id: ModelId::new(),
            filename: "cup.jpg".to_owned(),
            mime_type: "image/jpeg".to_owned(),
            image_bytes: vec![1, 2, 3],
            confidence_threshold: 0.25,
        })
        .await;

    assert_eq!(missing, Err(UseCaseError::NotFound("model not found")));

    let archived = models
        .create(model_fixture(ModelStatus::Archived))
        .await
        .expect("model is stored");
    let archived_result = RunInferenceUseCase::new(&models, &runs, &TestInferenceEngine)
        .execute(RunInferenceCommand {
            model_id: archived.id,
            filename: "cup.jpg".to_owned(),
            mime_type: "image/jpeg".to_owned(),
            image_bytes: vec![1, 2, 3],
            confidence_threshold: 0.25,
        })
        .await;

    assert_eq!(
        archived_result,
        Err(UseCaseError::Validation(
            "archived model cannot run inference"
        ))
    );
}

#[tokio::test]
async fn run_inference_rejects_invalid_image_contract() {
    let models = InMemoryModelRepository::default();
    let runs = InMemoryInferenceRunRepository::default();
    let model = models
        .create(model_fixture(ModelStatus::Candidate))
        .await
        .expect("model is stored");

    let result = RunInferenceUseCase::new(&models, &runs, &TestInferenceEngine)
        .execute(RunInferenceCommand {
            model_id: model.id,
            filename: "invalid.txt".to_owned(),
            mime_type: "text/plain".to_owned(),
            image_bytes: vec![1, 2, 3],
            confidence_threshold: 0.25,
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation("unsupported image mime type"))
    );
}
