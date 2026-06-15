use axum::{
    Json, Router,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use perception_app::{UploadSampleCommand, UploadSampleUseCase, UseCaseError};
use perception_domain::DatasetId;

use crate::{
    dto::{error::ErrorResponse, sample::SampleResponse},
    mappers,
    state::SampleHttpState,
};

pub fn routes(state: SampleHttpState) -> Router {
    Router::new()
        .route("/datasets/{dataset_id}/samples", post(upload_sample))
        .with_state(state)
}

async fn upload_sample(
    State(state): State<SampleHttpState>,
    Path(dataset_id): Path<String>,
    multipart: Multipart,
) -> Result<(StatusCode, Json<SampleResponse>), SampleRouteError> {
    let dataset_id = DatasetId::parse(dataset_id)
        .map_err(|_| UseCaseError::Validation("invalid dataset id"))?;
    let payload = read_upload_payload(multipart).await?;
    let use_case = UploadSampleUseCase::new(
        state.dataset_repository(),
        state.sample_repository(),
        state.sample_storage(),
    );
    let sample = use_case
        .execute(UploadSampleCommand {
            dataset_id,
            filename: payload.filename,
            mime_type: payload.mime_type,
            width: payload.width,
            height: payload.height,
            bytes: payload.bytes,
        })
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(mappers::sample::sample_response(sample)),
    ))
}

struct UploadPayload {
    filename: String,
    mime_type: String,
    width: u32,
    height: u32,
    bytes: Vec<u8>,
}

async fn read_upload_payload(mut multipart: Multipart) -> Result<UploadPayload, UseCaseError> {
    let mut filename = None;
    let mut mime_type = None;
    let mut width = None;
    let mut height = None;
    let mut bytes = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| UseCaseError::Validation("invalid multipart payload"))?
    {
        let field_name = field.name().unwrap_or_default().to_owned();

        match field_name.as_str() {
            "file" => {
                filename = field.file_name().map(str::to_owned);
                mime_type = field.content_type().map(ToString::to_string);
                bytes = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| UseCaseError::Validation("invalid multipart file"))?
                        .to_vec(),
                );
            }
            "width" => {
                width = Some(read_u32_field(field, "image width is required").await?);
            }
            "height" => {
                height = Some(read_u32_field(field, "image height is required").await?);
            }
            _ => {}
        }
    }

    Ok(UploadPayload {
        filename: filename.ok_or(UseCaseError::Validation("sample filename is required"))?,
        mime_type: mime_type.ok_or(UseCaseError::Validation("unsupported image mime type"))?,
        width: width.ok_or(UseCaseError::Validation("image width is required"))?,
        height: height.ok_or(UseCaseError::Validation("image height is required"))?,
        bytes: bytes.ok_or(UseCaseError::Validation("sample file is required"))?,
    })
}

async fn read_u32_field(
    field: axum::extract::multipart::Field<'_>,
    error: &'static str,
) -> Result<u32, UseCaseError> {
    let text = field
        .text()
        .await
        .map_err(|_| UseCaseError::Validation(error))?;

    text.trim()
        .parse::<u32>()
        .map_err(|_| UseCaseError::Validation(error))
}

struct SampleRouteError {
    error: UseCaseError,
}

impl From<UseCaseError> for SampleRouteError {
    fn from(error: UseCaseError) -> Self {
        Self { error }
    }
}

impl IntoResponse for SampleRouteError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self.error {
            UseCaseError::Validation(message) => {
                (StatusCode::BAD_REQUEST, "validation_failed", message.to_owned())
            }
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
