use std::collections::BTreeMap;

use perception_app::{SampleDraft, SampleRepository};
use perception_domain::{DatasetId, SampleId};
use perception_infra::TransientSampleRepository;

#[tokio::test]
async fn transient_sample_repository_creates_gets_and_lists_samples_by_dataset() {
    let repository = TransientSampleRepository::default();
    let dataset_id = DatasetId::new();
    let sample_id = SampleId::new();

    repository
        .create(SampleDraft {
            id: sample_id,
            dataset_id,
            storage_uri: "file:///tmp/cup.png".to_owned(),
            filename: "cup.png".to_owned(),
            mime_type: "image/png".to_owned(),
            width: 640,
            height: 480,
            size_bytes: 14,
            checksum: "sha256:test".to_owned(),
            source: "upload".to_owned(),
            metadata: BTreeMap::new(),
        })
        .await
        .expect("sample is persisted");

    assert_eq!(
        repository
            .get(sample_id)
            .await
            .expect("sample lookup succeeds")
            .expect("sample exists")
            .filename,
        "cup.png"
    );
    assert_eq!(
        repository
            .list_by_dataset(dataset_id)
            .await
            .expect("dataset samples are listed")
            .len(),
        1
    );
}
