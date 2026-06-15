use perception_domain::{
    ArtifactId, DatasetId, DatasetStatus, DomainError, ExportStatus, ImageDimensions, ModelStatus,
    NormalizedBbox, TrainingHyperparameters, TrainingJobStatus,
};

#[test]
fn newtype_ids_round_trip_through_strings() {
    let dataset_id = DatasetId::new();
    let artifact_id = ArtifactId::parse(dataset_id.to_string()).expect("uuid string parses");

    assert_eq!(dataset_id.to_string(), artifact_id.to_string());
}

#[test]
fn normalized_bbox_rejects_invalid_bounds() {
    assert_eq!(
        NormalizedBbox::new(0.9, 0.2, 0.2, 0.2),
        Err(DomainError::InvalidNormalizedBbox)
    );
    assert!(NormalizedBbox::new(0.1, 0.2, 0.3, 0.4).is_ok());
}

#[test]
fn image_dimensions_must_be_non_zero() {
    assert_eq!(
        ImageDimensions::new(0, 640),
        Err(DomainError::InvalidImageDimensions)
    );
    assert_eq!(
        ImageDimensions::new(640, 480)
            .expect("valid dimensions")
            .width,
        640
    );
}

#[test]
fn training_hyperparameters_reject_zero_values() {
    assert_eq!(
        TrainingHyperparameters::new(0, 16, 640, 0.001),
        Err(DomainError::InvalidTrainingHyperparameters)
    );
    assert!(TrainingHyperparameters::new(1, 1, 320, 0.001).is_ok());
}

#[test]
fn dataset_status_transitions_are_explicit() {
    assert_eq!(
        DatasetStatus::Draft.transition_to(DatasetStatus::Ready),
        Ok(DatasetStatus::Ready)
    );
    assert_eq!(
        DatasetStatus::Archived.transition_to(DatasetStatus::Ready),
        Err(DomainError::InvalidStatusTransition)
    );
}

#[test]
fn training_job_status_transitions_are_guarded() {
    assert_eq!(
        TrainingJobStatus::Queued.transition_to(TrainingJobStatus::Running),
        Ok(TrainingJobStatus::Running)
    );
    assert_eq!(
        TrainingJobStatus::Succeeded.transition_to(TrainingJobStatus::Running),
        Err(DomainError::InvalidStatusTransition)
    );
}

#[test]
fn model_status_transitions_are_guarded() {
    assert_eq!(
        ModelStatus::Candidate.transition_to(ModelStatus::Validated),
        Ok(ModelStatus::Validated)
    );
    assert_eq!(
        ModelStatus::Archived.transition_to(ModelStatus::Promoted),
        Err(DomainError::InvalidStatusTransition)
    );
}

#[test]
fn export_status_transitions_are_guarded() {
    assert_eq!(
        ExportStatus::Queued.transition_to(ExportStatus::Running),
        Ok(ExportStatus::Running)
    );
    assert_eq!(
        ExportStatus::Succeeded.transition_to(ExportStatus::Failed),
        Err(DomainError::InvalidStatusTransition)
    );
}
