use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SampleResponse {
    pub id: String,
    pub dataset_id: String,
    pub storage_uri: String,
    pub filename: String,
    pub mime_type: String,
    pub width: u32,
    pub height: u32,
    pub size_bytes: u64,
    pub checksum: String,
    pub source: String,
}
