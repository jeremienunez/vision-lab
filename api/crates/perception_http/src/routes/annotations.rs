use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use perception_app::{
    AddAnnotationCommand, AddAnnotationUseCase, ListSampleAnnotationsUseCase, UseCaseError,
};
use perception_domain::SampleId;

use crate::{
    dto::{
        annotation::{AddAnnotationRequest, AnnotationResponse, ListAnnotationsResponse},
        error::ErrorResponse,
    },
    mappers,
    state::AnnotationHttpState,
};

pub fn routes(state: AnnotationHttpState) -> Router {
    Router::new()
        .route(
            "/samples/{sample_id}/annotations",
            post(add_annotation).get(list_annotations),
        )
        .with_state(state)
}

async fn add_annotation(
    State(state): State<AnnotationHttpState>,
    Path(sample_id): Path<String>,
    Json(request): Json<AddAnnotationRequest>,
) -> Result<(StatusCode, Json<AnnotationResponse>), AnnotationRouteError> {
    let sample_id =
        SampleId::parse(sample_id).map_err(|_| UseCaseError::Validation("invalid sample id"))?;
    let use_case = AddAnnotationUseCase::new(
        state.dataset_repository(),
        state.sample_repository(),
        state.annotation_repository(),
    );
    let annotation = use_case
        .execute(AddAnnotationCommand {
            sample_id,
            class_name: request.class_name,
            bbox_x: request.bbox.x,
            bbox_y: request.bbox.y,
            bbox_width: request.bbox.width,
            bbox_height: request.bbox.height,
            confidence: request.confidence,
        })
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(mappers::annotation::annotation_response(annotation)),
    ))
}

async fn list_annotations(
    State(state): State<AnnotationHttpState>,
    Path(sample_id): Path<String>,
) -> Result<Json<ListAnnotationsResponse>, AnnotationRouteError> {
    let sample_id =
        SampleId::parse(sample_id).map_err(|_| UseCaseError::Validation("invalid sample id"))?;
    let use_case =
        ListSampleAnnotationsUseCase::new(state.sample_repository(), state.annotation_repository());
    let annotations = use_case.execute(sample_id).await?;

    Ok(Json(ListAnnotationsResponse {
        annotations: annotations
            .into_iter()
            .map(mappers::annotation::annotation_response)
            .collect(),
    }))
}

struct AnnotationRouteError {
    error: UseCaseError,
}

impl From<UseCaseError> for AnnotationRouteError {
    fn from(error: UseCaseError) -> Self {
        Self { error }
    }
}

impl IntoResponse for AnnotationRouteError {
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
