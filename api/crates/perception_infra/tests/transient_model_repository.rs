use std::collections::BTreeMap;

use perception_app::{ModelDraft, ModelRepository};
use perception_domain::{DatasetVersionId, ModelId, ModelStatus, TrainingJobId};
use perception_infra::TransientModelRepository;

fn model_fixture(name: &str) -> ModelDraft {
    ModelDraft {
        id: ModelId::new(),
        name: name.to_owned(),
        version: "v1".to_owned(),
        training_job_id: TrainingJobId::new(),
        dataset_version_id: DatasetVersionId::new(),
        model_family: "tiny_torch".to_owned(),
        artifact_uri: "file:///tmp/model.pt".to_owned(),
        metrics_summary: BTreeMap::new(),
        status: ModelStatus::Candidate,
    }
}

#[tokio::test]
async fn transient_model_repository_creates_lists_and_gets_models() {
    let repository = TransientModelRepository::default();
    let model = repository
        .create(model_fixture("desk-objects"))
        .await
        .expect("model is stored");

    let listed = repository.list().await.expect("models are listed");
    let fetched = repository
        .get(model.id)
        .await
        .expect("model lookup succeeds")
        .expect("model exists");

    assert_eq!(listed, vec![model.clone()]);
    assert_eq!(fetched, model);
}

#[tokio::test]
async fn transient_model_repository_updates_existing_model() {
    let repository = TransientModelRepository::default();
    let mut model = repository
        .create(model_fixture("desk-objects"))
        .await
        .expect("model is stored");
    model.status = ModelStatus::Promoted;

    let updated = repository
        .update(model.clone())
        .await
        .expect("model is updated");
    let fetched = repository
        .get(model.id)
        .await
        .expect("model lookup succeeds")
        .expect("model exists");

    assert_eq!(updated.status, ModelStatus::Promoted);
    assert_eq!(fetched, model);
}
