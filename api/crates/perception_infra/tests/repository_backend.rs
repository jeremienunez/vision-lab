use perception_infra::RepositoryBackend;

#[test]
fn repository_backend_defaults_to_transient_for_missing_or_unknown_values() {
    assert_eq!(
        RepositoryBackend::from_env_value(None),
        RepositoryBackend::Transient
    );
    assert_eq!(
        RepositoryBackend::from_env_value(Some("memory")),
        RepositoryBackend::Transient
    );
}

#[test]
fn repository_backend_accepts_postgres_aliases() {
    assert_eq!(
        RepositoryBackend::from_env_value(Some("postgres")),
        RepositoryBackend::Postgres
    );
    assert_eq!(
        RepositoryBackend::from_env_value(Some("sqlx")),
        RepositoryBackend::Postgres
    );
}
