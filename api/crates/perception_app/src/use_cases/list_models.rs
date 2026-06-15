use crate::{ModelDraft, ModelRepository, UseCaseError};

pub struct ListModelsUseCase<'repository> {
    model_repository: &'repository dyn ModelRepository,
}

impl<'repository> ListModelsUseCase<'repository> {
    pub fn new(model_repository: &'repository dyn ModelRepository) -> Self {
        Self { model_repository }
    }

    pub async fn execute(&self) -> Result<Vec<ModelDraft>, UseCaseError> {
        self.model_repository.list().await
    }
}
