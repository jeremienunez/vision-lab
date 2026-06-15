use std::collections::BTreeMap;

use perception_app::{DatasetVersionDraft, DatasetVersionRepository};
use perception_domain::{DatasetId, DatasetVersionId};
use perception_infra::TransientDatasetVersionRepository;

#[tokio::test]
async fn transient_dataset_version_repository_creates_and_gets_versions() {
    let repository = TransientDatasetVersionRepository::default();
    let version_id = DatasetVersionId::new();

    repository
        .create(DatasetVersionDraft {
            id: version_id,
            dataset_id: DatasetId::new(),
            version_name: "v1".to_owned(),
            sample_count: 1,
            annotation_count: 0,
            classes_snapshot: vec!["cup".to_owned()],
            split_config: BTreeMap::new(),
            created_by: "local-user".to_owned(),
        })
        .await
        .expect("version is persisted");

    assert_eq!(
        repository
            .get(version_id)
            .await
            .expect("version lookup succeeds")
            .expect("version exists")
            .version_name,
        "v1"
    );
}
