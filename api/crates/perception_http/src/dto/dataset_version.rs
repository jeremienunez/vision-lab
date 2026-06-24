use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateDatasetVersionRequest {
    pub version_name: String,
    #[serde(default)]
    pub split_config: BTreeMap<String, String>,
    pub created_by: String,
}

#[derive(Debug, Serialize)]
pub struct DatasetVersionResponse {
    pub id: String,
    pub dataset_id: String,
    pub version_name: String,
    pub sample_count: u64,
    pub annotation_count: u64,
    pub classes_snapshot: Vec<String>,
    pub split_config: BTreeMap<String, String>,
    pub created_by: String,
}

#[derive(Debug, Serialize)]
pub struct ListDatasetVersionsResponse {
    pub dataset_versions: Vec<DatasetVersionResponse>,
}
