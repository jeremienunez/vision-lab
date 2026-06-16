use perception_app::DatasetVersionDraft;

use crate::dto::dataset_version::DatasetVersionResponse;

pub fn dataset_version_response(version: DatasetVersionDraft) -> DatasetVersionResponse {
    DatasetVersionResponse {
        id: version.id.to_string(),
        dataset_id: version.dataset_id.to_string(),
        version_name: version.version_name,
        sample_count: version.sample_count,
        annotation_count: version.annotation_count,
        classes_snapshot: version.classes_snapshot,
        split_config: version.split_config,
        created_by: version.created_by,
    }
}
