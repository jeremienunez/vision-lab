use perception_app::{DetectionDraft, InferenceResult};

use crate::dto::inference::{DetectionBboxResponse, DetectionResponse, InferenceResponse};

pub fn inference_response(result: InferenceResult) -> InferenceResponse {
    InferenceResponse {
        model_id: result.model_id.to_string(),
        latency_ms: result.latency_ms,
        detections: result
            .detections
            .into_iter()
            .map(detection_response)
            .collect(),
    }
}

fn detection_response(detection: DetectionDraft) -> DetectionResponse {
    DetectionResponse {
        class_id: detection.class_id,
        class_name: detection.class_name,
        confidence: detection.confidence,
        bbox: DetectionBboxResponse {
            x: detection.bbox.x,
            y: detection.bbox.y,
            width: detection.bbox.width,
            height: detection.bbox.height,
        },
        distance_m: detection.distance_m,
    }
}
