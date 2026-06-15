use perception_domain::ModelId;

use crate::{ModelExportDraft, ModelExportRepository, ModelRepository, UseCaseError};

pub struct ListModelExportsUseCase<'repository> {
    model_repository: &'repository dyn ModelRepository,
    export_repository: &'repository dyn ModelExportRepository,
}

impl<'repository> ListModelExportsUseCase<'repository> {
    pub fn new(
        model_repository: &'repository dyn ModelRepository,
        export_repository: &'repository dyn ModelExportRepository,
    ) -> Self {
        Self {
            model_repository,
            export_repository,
        }
    }

    pub async fn execute(&self, model_id: ModelId) -> Result<Vec<ModelExportDraft>, UseCaseError> {
        self.model_repository
            .get(model_id)
            .await?
            .ok_or(UseCaseError::NotFound("model not found"))?;

        self.export_repository.list_by_model(model_id).await
    }
}
