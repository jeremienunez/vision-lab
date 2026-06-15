use perception_app::{ModelExportDraft, ModelExportRepository};
use perception_domain::{ExportStatus, ModelExportId, ModelId};

fn export_fixture(model_id: ModelId, format: &str) -> ModelExportDraft {
    ModelExportDraft {
        id: ModelExportId::new(),
        model_id,
        format: format.to_owned(),
        artifact_uri: Some(format!("file:///tmp/model.{format}")),
        status: ExportStatus::Succeeded,
        error_message: None,
    }
}

#[tokio::test]
async fn transient_model_export_repository_creates_and_lists_exports_by_model() {
    let repository = perception_infra::TransientModelExportRepository::default();
    let model = ModelId::new();
    let other_model = ModelId::new();
    let export = repository
        .create(export_fixture(model, "onnx"))
        .await
        .expect("export is stored");
    repository
        .create(export_fixture(other_model, "onnx"))
        .await
        .expect("other export is stored");

    let listed = repository
        .list_by_model(model)
        .await
        .expect("exports are listed by model");

    assert_eq!(listed, vec![export]);
}
