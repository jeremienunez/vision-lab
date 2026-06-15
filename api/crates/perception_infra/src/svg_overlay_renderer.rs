use std::path::{Path, PathBuf};

use async_trait::async_trait;
use perception_app::{InferenceRunDraft, OverlayArtifact, OverlayRenderer, UseCaseError};
use tokio::fs;

pub struct SvgOverlayRenderer {
    root: PathBuf,
}

impl SvgOverlayRenderer {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }
}

#[async_trait]
impl OverlayRenderer for SvgOverlayRenderer {
    async fn render(&self, run: InferenceRunDraft) -> Result<OverlayArtifact, UseCaseError> {
        fs::create_dir_all(&self.root)
            .await
            .map_err(|_| UseCaseError::Repository("overlay root unavailable"))?;
        let artifact_path = self.root.join(format!("{}.svg", run.id));
        let labels = run
            .detections
            .iter()
            .map(|detection| {
                format!(
                    "{} {:.0}%",
                    detection.class_name,
                    detection.confidence * 100.0
                )
            })
            .collect::<Vec<_>>();
        let svg = overlay_svg(&run, &labels);

        fs::write(&artifact_path, svg)
            .await
            .map_err(|_| UseCaseError::Repository("overlay artifact write failed"))?;
        let artifact_path = fs::canonicalize(&artifact_path)
            .await
            .map_err(|_| UseCaseError::Repository("overlay artifact path unavailable"))?;

        Ok(OverlayArtifact {
            inference_run_id: run.id,
            artifact_uri: format!("file://{}", artifact_path.display()),
            labels,
        })
    }
}

fn overlay_svg(run: &InferenceRunDraft, labels: &[String]) -> String {
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1000 1000" role="img" aria-label="PerceptionLab overlay for {}">"#,
        escape_xml(&run.filename)
    );
    svg.push_str(r##"<rect x="0" y="0" width="1000" height="1000" fill="#111827"/>"##);

    for (detection, label) in run.detections.iter().zip(labels.iter()) {
        let x = detection.bbox.x * 1000.0;
        let y = detection.bbox.y * 1000.0;
        let width = detection.bbox.width * 1000.0;
        let height = detection.bbox.height * 1000.0;
        svg.push_str(&format!(
            r##"<rect x="{x:.1}" y="{y:.1}" width="{width:.1}" height="{height:.1}" fill="none" stroke="#22c55e" stroke-width="8"/>"##
        ));
        svg.push_str(&format!(
            r##"<text x="{x:.1}" y="{text_y:.1}" fill="#ecfdf5" font-family="Arial, sans-serif" font-size="42">{label}</text>"##,
            text_y = (y - 12.0).max(48.0),
            label = escape_xml(label)
        ));
    }

    svg.push_str("</svg>");
    svg
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
