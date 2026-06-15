use perception_app::SampleDraft;

use crate::dto::sample::SampleResponse;

pub fn sample_response(sample: SampleDraft) -> SampleResponse {
    SampleResponse {
        id: sample.id.to_string(),
        dataset_id: sample.dataset_id.to_string(),
        storage_uri: sample.storage_uri,
        filename: sample.filename,
        mime_type: sample.mime_type,
        width: sample.width,
        height: sample.height,
        size_bytes: sample.size_bytes,
        checksum: sample.checksum,
        source: sample.source,
    }
}
