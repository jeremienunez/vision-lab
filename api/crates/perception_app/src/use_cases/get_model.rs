use perception_domain::ModelId;

use crate::{ModelDraft, ModelRepository, UseCaseError};

pub struct GetModelUseCase<'repository> {
    model_repository: &'repository dyn ModelRepository,
}

impl<'repository> GetModelUseCase<'repository> {
    pub fn new(model_repository: &'repository dyn ModelRepository) -> Self {
        Self { model_repository }
    }

    pub async fn execute(&self, model_id: ModelId) -> Result<ModelDraft, UseCaseError> {
        self.model_repository
            .get(model_id)
            .await?
            .ok_or(UseCaseError::NotFound("model not found"))
    }
}
