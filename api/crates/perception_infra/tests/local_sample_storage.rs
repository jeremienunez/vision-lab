use std::{fs, path::PathBuf};

use perception_app::{SampleStorage, SampleStorageCommand};
use perception_domain::{DatasetId, SampleId};
use perception_infra::LocalSampleStorage;

fn storage_root() -> PathBuf {
    std::env::temp_dir().join(format!("perceptionlab-storage-test-{}", SampleId::new()))
}

#[tokio::test]
async fn local_sample_storage_writes_bytes_and_returns_file_uri_with_checksum() {
    let root = storage_root();
    let storage = LocalSampleStorage::new(&root);
    let dataset_id = DatasetId::new();
    let sample_id = SampleId::new();

    let stored = storage
        .store(SampleStorageCommand {
            dataset_id,
            sample_id,
            filename: "cup.png".to_owned(),
            mime_type: "image/png".to_owned(),
            bytes: b"fake-png-bytes".to_vec(),
        })
        .await
        .expect("sample is stored");

    assert!(stored.storage_uri.starts_with("file://"));
    assert_eq!(stored.size_bytes, 14);
    assert_eq!(
        stored.checksum,
        "sha256:3c6ed5fc41c950bf0db531eb22f945467fb8d999f80d82ba27dcc9fd90add54d"
    );
    assert!(fs::read_dir(&root).expect("root exists").next().is_some());

    fs::remove_dir_all(root).expect("temporary storage is removed");
}
