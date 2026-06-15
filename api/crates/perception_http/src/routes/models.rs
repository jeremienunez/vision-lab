use axum::{
    Json, Router,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use perception_app::{
    ExportModelCommand, ExportModelUseCase, GetModelUseCase, ListModelExportsUseCase,
    ListModelsUseCase, RunInferenceCommand, RunInferenceUseCase, UseCaseError,
};
use perception_domain::ModelId;

use crate::{
    dto::{
        error::ErrorResponse,
        inference::InferenceResponse,
        model::{ListModelsResponse, ModelResponse},
        model_export::{CreateModelExportRequest, ListModelExportsResponse, ModelExportResponse},
    },
    mappers,
    state::ModelHttpState,
};

pub fn routes(state: ModelHttpState) -> Router {
    Router::new()
        .route("/models", get(list_models))
        .route("/models/{model_id}", get(get_model))
        .route("/models/{model_id}/infer", post(run_inference))
        .route(
            "/models/{model_id}/exports",
            post(export_model).get(list_model_exports),
        )
        .with_state(state)
}

async fn list_models(
    State(state): State<ModelHttpState>,
) -> Result<Json<ListModelsResponse>, ModelRouteError> {
    let models = ListModelsUseCase::new(state.model_repository())
        .execute()
        .await?;

    Ok(Json(ListModelsResponse {
        models: models
            .into_iter()
            .map(mappers::model::model_response)
            .collect(),
    }))
}

async fn get_model(
    State(state): State<ModelHttpState>,
    Path(model_id): Path<String>,
) -> Result<Json<ModelResponse>, ModelRouteError> {
    let model_id =
        ModelId::parse(model_id).map_err(|_| UseCaseError::Validation("invalid model id"))?;
    let model = GetModelUseCase::new(state.model_repository())
        .execute(model_id)
        .await?;

    Ok(Json(mappers::model::model_response(model)))
}

async fn run_inference(
    State(state): State<ModelHttpState>,
    Path(model_id): Path<String>,
    multipart: Multipart,
) -> Result<Json<InferenceResponse>, ModelRouteError> {
    let model_id =
        ModelId::parse(model_id).map_err(|_| UseCaseError::Validation("invalid model id"))?;
    let payload = read_inference_payload(multipart).await?;
    let result = RunInferenceUseCase::new(
        state.model_repository(),
        state.inference_run_repository(),
        state.inference_engine(),
    )
    .execute(RunInferenceCommand {
        model_id,
        filename: payload.filename,
        mime_type: payload.mime_type,
        image_bytes: payload.image_bytes,
        confidence_threshold: payload.confidence_threshold,
    })
    .await?;

    Ok(Json(mappers::inference::inference_response(result)))
}

async fn export_model(
    State(state): State<ModelHttpState>,
    Path(model_id): Path<String>,
    Json(request): Json<CreateModelExportRequest>,
) -> Result<(StatusCode, Json<ModelExportResponse>), ModelRouteError> {
    let model_id =
        ModelId::parse(model_id).map_err(|_| UseCaseError::Validation("invalid model id"))?;
    let export = ExportModelUseCase::new(state.model_repository(), state.model_export_repository())
        .execute(ExportModelCommand {
            model_id,
            format: request.format,
        })
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(mappers::model_export::model_export_response(export)),
    ))
}

async fn list_model_exports(
    State(state): State<ModelHttpState>,
    Path(model_id): Path<String>,
) -> Result<Json<ListModelExportsResponse>, ModelRouteError> {
    let model_id =
        ModelId::parse(model_id).map_err(|_| UseCaseError::Validation("invalid model id"))?;
    let exports =
        ListModelExportsUseCase::new(state.model_repository(), state.model_export_repository())
            .execute(model_id)
            .await?;

    Ok(Json(ListModelExportsResponse {
        exports: exports
            .into_iter()
            .map(mappers::model_export::model_export_response)
            .collect(),
    }))
}

struct InferencePayload {
    filename: String,
    mime_type: String,
    image_bytes: Vec<u8>,
    confidence_threshold: f32,
}

async fn read_inference_payload(
    mut multipart: Multipart,
) -> Result<InferencePayload, UseCaseError> {
    let mut filename = None;
    let mut mime_type = None;
    let mut image_bytes = None;
    let mut confidence_threshold = 0.25;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| UseCaseError::Validation("invalid multipart payload"))?
    {
        let field_name = field.name().unwrap_or_default().to_owned();

        match field_name.as_str() {
            "image" => {
                filename = field.file_name().map(str::to_owned);
                mime_type = field.content_type().map(ToString::to_string);
                image_bytes = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| UseCaseError::Validation("invalid multipart image"))?
                        .to_vec(),
                );
            }
            "confidence_threshold" => {
                confidence_threshold =
                    read_f32_field(field, "invalid confidence threshold").await?;
            }
            _ => {}
        }
    }

    Ok(InferencePayload {
        filename: filename.ok_or(UseCaseError::Validation("invalid inference image"))?,
        mime_type: mime_type.ok_or(UseCaseError::Validation("unsupported image mime type"))?,
        image_bytes: image_bytes.ok_or(UseCaseError::Validation("invalid inference image"))?,
        confidence_threshold,
    })
}

async fn read_f32_field(
    field: axum::extract::multipart::Field<'_>,
    error: &'static str,
) -> Result<f32, UseCaseError> {
    let text = field
        .text()
        .await
        .map_err(|_| UseCaseError::Validation(error))?;

    text.trim()
        .parse::<f32>()
        .map_err(|_| UseCaseError::Validation(error))
}

struct ModelRouteError {
    error: UseCaseError,
}

impl From<UseCaseError> for ModelRouteError {
    fn from(error: UseCaseError) -> Self {
        Self { error }
    }
}

impl IntoResponse for ModelRouteError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self.error {
            UseCaseError::Validation("unsupported image mime type") => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "unsupported_media_type",
                "unsupported image mime type".to_owned(),
            ),
            UseCaseError::Validation(message) => (
                StatusCode::BAD_REQUEST,
                "validation_failed",
                message.to_owned(),
            ),
            UseCaseError::NotFound(message) => {
                (StatusCode::NOT_FOUND, "not_found", message.to_owned())
            }
            UseCaseError::Repository(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "repository_failed",
                message.to_owned(),
            ),
        };

        (status, Json(ErrorResponse::new(code, message))).into_response()
    }
}
