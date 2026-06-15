use perception_domain::InferenceRunId;

use crate::{InferenceRunRepository, OverlayArtifact, OverlayRenderer, UseCaseError};

pub struct GenerateOverlayUseCase<'repository> {
    inference_run_repository: &'repository dyn InferenceRunRepository,
    overlay_renderer: &'repository dyn OverlayRenderer,
}

impl<'repository> GenerateOverlayUseCase<'repository> {
    pub fn new(
        inference_run_repository: &'repository dyn InferenceRunRepository,
        overlay_renderer: &'repository dyn OverlayRenderer,
    ) -> Self {
        Self {
            inference_run_repository,
            overlay_renderer,
        }
    }

    pub async fn execute(
        &self,
        inference_run_id: InferenceRunId,
    ) -> Result<OverlayArtifact, UseCaseError> {
        let run = self
            .inference_run_repository
            .get(inference_run_id)
            .await?
            .ok_or(UseCaseError::NotFound("inference run not found"))?;

        self.overlay_renderer.render(run).await
    }
}
