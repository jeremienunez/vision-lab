#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryBackend {
    Transient,
    Postgres,
}

impl RepositoryBackend {
    pub fn from_env_value(value: Option<&str>) -> Self {
        match value {
            Some("postgres" | "sqlx") => Self::Postgres,
            _ => Self::Transient,
        }
    }

    pub fn from_env() -> Self {
        Self::from_env_value(
            std::env::var("PERCEPTIONLAB_REPOSITORY_BACKEND")
                .ok()
                .as_deref(),
        )
    }
}
