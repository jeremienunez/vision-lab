use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DatasetStatsResponse {
    pub dataset_id: String,
    pub sample_count: u64,
    pub annotation_count: u64,
    pub annotations_by_class: BTreeMap<String, u64>,
}
