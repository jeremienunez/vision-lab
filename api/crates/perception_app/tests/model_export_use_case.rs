use std::{collections::BTreeMap, sync::Mutex};

use async_trait::async_trait;
use perception_app::{
    ExportModelCommand, ExportModelUseCase, ListModelExportsUseCase, ModelDraft, ModelExportDraft,
    ModelExportRepository, ModelRepository, UseCaseError,
};
use perception_domain::{
    DatasetVersionId, ExportStatus, ModelExportId, ModelId, ModelStatus, TrainingJobId,
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
struct InMemoryModelExportRepository {
    exports: Mutex<Vec<ModelExportDraft>>,
}

#[async_trait]
impl ModelExportRepository for InMemoryModelExportRepository {
    async fn create(&self, export: ModelExportDraft) -> Result<ModelExportDraft, UseCaseError> {
        self.exports
            .lock()
            .expect("repository mutex is available")
            .push(export.clone());
        Ok(export)
    }

    async fn list_by_model(
        &self,
        model_id: ModelId,
    ) -> Result<Vec<ModelExportDraft>, UseCaseError> {
        Ok(self
            .exports
            .lock()
            .expect("repository mutex is available")
            .iter()
            .filter(|export| export.model_id == model_id)
            .cloned()
            .collect())
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
async fn export_model_creates_succeeded_onnx_export_for_existing_model() {
    let models = InMemoryModelRepository::default();
    let exports = InMemoryModelExportRepository::default();
    let model = models
        .create(model_fixture(ModelStatus::Candidate))
        .await
        .expect("model is created");

    let export = ExportModelUseCase::new(&models, &exports)
        .execute(ExportModelCommand {
            model_id: model.id,
            format: "onnx".to_owned(),
        })
        .await
        .expect("model export is created");

    let _export_id: ModelExportId = export.id;
    assert_eq!(export.model_id, model.id);
    assert_eq!(export.format, "onnx");
    assert_eq!(
        export.artifact_uri,
        Some("file:///tmp/model.onnx".to_owned())
    );
    assert_eq!(export.status, ExportStatus::Succeeded);
    assert_eq!(export.error_message, None);

    let listed = ListModelExportsUseCase::new(&models, &exports)
        .execute(model.id)
        .await
        .expect("model exports are listed");

    assert_eq!(listed, vec![export]);
}

#[tokio::test]
async fn export_model_rejects_unsupported_format() {
    let models = InMemoryModelRepository::default();
    let exports = InMemoryModelExportRepository::default();
    let model = models
        .create(model_fixture(ModelStatus::Candidate))
        .await
        .expect("model is created");

    let result = ExportModelUseCase::new(&models, &exports)
        .execute(ExportModelCommand {
            model_id: model.id,
            format: "tflite".to_owned(),
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation("unsupported model export format"))
    );
}

#[tokio::test]
async fn export_model_creates_succeeded_coreml_export_for_existing_model() {
    let models = InMemoryModelRepository::default();
    let exports = InMemoryModelExportRepository::default();
    let model = models
        .create(model_fixture(ModelStatus::Validated))
        .await
        .expect("model is created");

    let export = ExportModelUseCase::new(&models, &exports)
        .execute(ExportModelCommand {
            model_id: model.id,
            format: "coreml".to_owned(),
        })
        .await
        .expect("model export is created");

    assert_eq!(export.model_id, model.id);
    assert_eq!(export.format, "coreml");
    assert_eq!(
        export.artifact_uri,
        Some("file:///tmp/model.mlpackage".to_owned())
    );
    assert_eq!(export.status, ExportStatus::Succeeded);
    assert_eq!(export.error_message, None);
}

#[tokio::test]
async fn export_model_rejects_archived_model() {
    let models = InMemoryModelRepository::default();
    let exports = InMemoryModelExportRepository::default();
    let model = models
        .create(model_fixture(ModelStatus::Archived))
        .await
        .expect("model is created");

    let result = ExportModelUseCase::new(&models, &exports)
        .execute(ExportModelCommand {
            model_id: model.id,
            format: "onnx".to_owned(),
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation(
            "archived model cannot be exported"
        ))
    );
}
