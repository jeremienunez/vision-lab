use perception_app::AnnotationDraft;

use crate::dto::annotation::{AnnotationResponse, BboxResponse};

pub fn annotation_response(annotation: AnnotationDraft) -> AnnotationResponse {
    AnnotationResponse {
        id: annotation.id.to_string(),
        sample_id: annotation.sample_id.to_string(),
        dataset_id: annotation.dataset_id.to_string(),
        class_name: annotation.class_name,
        class_id: annotation.class_id,
        bbox: BboxResponse {
            x: annotation.bbox_x,
            y: annotation.bbox_y,
            width: annotation.bbox_width,
            height: annotation.bbox_height,
        },
        format: annotation.format,
        confidence: annotation.confidence,
        source: annotation.source,
    }
}
