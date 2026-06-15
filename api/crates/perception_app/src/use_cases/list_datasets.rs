use crate::{DatasetDraft, DatasetRepository, UseCaseError};

pub struct ListDatasetsUseCase<'repository> {
    repository: &'repository dyn DatasetRepository,
}

impl<'repository> ListDatasetsUseCase<'repository> {
    pub fn new(repository: &'repository dyn DatasetRepository) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<DatasetDraft>, UseCaseError> {
        self.repository.list().await
    }
}
