use std::sync::Arc;

use perception_app::DatasetRepository;

#[derive(Clone)]
pub struct HttpState {
    dataset_repository: Arc<dyn DatasetRepository>,
}

impl HttpState {
    pub fn new(dataset_repository: Arc<dyn DatasetRepository>) -> Self {
        Self { dataset_repository }
    }

    pub fn dataset_repository(&self) -> &dyn DatasetRepository {
        self.dataset_repository.as_ref()
    }
}
