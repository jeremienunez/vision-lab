use perception_domain::{ModelId, ModelStatus};

use crate::{ModelDraft, ModelRepository, UseCaseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromoteModelCommand {
    pub model_id: ModelId,
}

pub struct PromoteModelUseCase<'repository> {
    model_repository: &'repository dyn ModelRepository,
}

impl<'repository> PromoteModelUseCase<'repository> {
    pub fn new(model_repository: &'repository dyn ModelRepository) -> Self {
        Self { model_repository }
    }

    pub async fn execute(&self, command: PromoteModelCommand) -> Result<ModelDraft, UseCaseError> {
        let mut target = self
            .model_repository
            .get(command.model_id)
            .await?
            .ok_or(UseCaseError::NotFound("model not found"))?;

        if target.status == ModelStatus::Archived {
            return Err(UseCaseError::Validation(
                "archived model cannot be promoted",
            ));
        }

        let competing_models = self.model_repository.list().await?;

        for mut competing_model in competing_models
            .into_iter()
            .filter(|model| model.id != target.id)
            .filter(|model| model.dataset_version_id == target.dataset_version_id)
            .filter(|model| model.model_family == target.model_family)
            .filter(|model| model.status == ModelStatus::Promoted)
        {
            competing_model.status = ModelStatus::Validated;
            self.model_repository.update(competing_model).await?;
        }

        target.status = ModelStatus::Promoted;
        self.model_repository.update(target).await
    }
}
