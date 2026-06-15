use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateDatasetRequest {
    pub name: String,
    pub description: Option<String>,
    pub task_type: String,
    pub classes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DatasetResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub task_type: &'static str,
    pub classes: Vec<String>,
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ListDatasetsResponse {
    pub datasets: Vec<DatasetResponse>,
}
