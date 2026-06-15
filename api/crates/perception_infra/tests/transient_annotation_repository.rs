use perception_app::{AnnotationDraft, AnnotationRepository};
use perception_domain::{AnnotationId, DatasetId, SampleId};
use perception_infra::TransientAnnotationRepository;

#[tokio::test]
async fn transient_annotation_repository_creates_and_lists_annotations_by_sample() {
    let repository = TransientAnnotationRepository::default();
    let sample_id = SampleId::new();
    let dataset_id = DatasetId::new();

    repository
        .create(AnnotationDraft {
            id: AnnotationId::new(),
            sample_id,
            dataset_id,
            class_name: "cup".to_owned(),
            class_id: 0,
            bbox_x: 0.10,
            bbox_y: 0.20,
            bbox_width: 0.30,
            bbox_height: 0.40,
            format: "normalized_xywh".to_owned(),
            confidence: Some(0.91),
            source: "manual".to_owned(),
        })
        .await
        .expect("annotation is persisted");

    let annotations = repository
        .list_by_sample(sample_id)
        .await
        .expect("annotations are listed");

    assert_eq!(annotations.len(), 1);
    assert_eq!(annotations[0].class_name, "cup");
    assert_eq!(
        repository
            .list_by_dataset(dataset_id)
            .await
            .expect("dataset annotations are listed")
            .len(),
        1
    );
}
