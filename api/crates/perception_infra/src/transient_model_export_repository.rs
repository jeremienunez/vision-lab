use std::sync::RwLock;

use async_trait::async_trait;
use perception_app::{ModelExportDraft, ModelExportRepository, UseCaseError};
use perception_domain::ModelId;

#[derive(Default)]
pub struct TransientModelExportRepository {
    exports: RwLock<Vec<ModelExportDraft>>,
}

#[async_trait]
impl ModelExportRepository for TransientModelExportRepository {
    async fn create(&self, export: ModelExportDraft) -> Result<ModelExportDraft, UseCaseError> {
        self.exports
            .write()
            .map_err(|_| UseCaseError::Repository("model export repository lock poisoned"))?
            .push(export.clone());

        Ok(export)
    }

    async fn list_by_model(
        &self,
        model_id: ModelId,
    ) -> Result<Vec<ModelExportDraft>, UseCaseError> {
        self.exports
            .read()
            .map(|exports| {
                exports
                    .iter()
                    .filter(|export| export.model_id == model_id)
                    .cloned()
                    .collect()
            })
            .map_err(|_| UseCaseError::Repository("model export repository lock poisoned"))
    }
}
