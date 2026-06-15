use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub dependencies: HealthDependencies,
}

#[derive(Debug, Serialize)]
pub struct HealthDependencies {
    pub database: &'static str,
    pub storage: &'static str,
    pub queue: &'static str,
}

impl HealthResponse {
    pub fn healthy_without_configured_dependencies() -> Self {
        Self {
            status: "healthy",
            dependencies: HealthDependencies {
                database: "not_configured",
                storage: "not_configured",
                queue: "not_configured",
            },
        }
    }
}
