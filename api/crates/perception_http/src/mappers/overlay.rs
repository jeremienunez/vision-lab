use perception_app::OverlayArtifact;

use crate::dto::overlay::OverlayResponse;

pub fn overlay_response(overlay: OverlayArtifact) -> OverlayResponse {
    OverlayResponse {
        inference_run_id: overlay.inference_run_id.to_string(),
        artifact_uri: overlay.artifact_uri,
        labels: overlay.labels,
    }
}
