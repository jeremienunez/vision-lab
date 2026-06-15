use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use perception_app::{
    AddAnnotationCommand, AddAnnotationUseCase, ExportYoloAnnotationsUseCase,
    ImportYoloAnnotationsUseCase, ListSampleAnnotationsUseCase, UseCaseError,
};
use perception_domain::{DatasetId, SampleId};

use crate::{
    dto::{
        annotation::{AddAnnotationRequest, AnnotationResponse, ListAnnotationsResponse},
        error::ErrorResponse,
        yolo_export::YoloAnnotationExportResponse,
        yolo_import::{YoloAnnotationImportRequest, YoloAnnotationImportResponse},
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
        .route("/datasets/{dataset_id}/import/yolo", post(import_yolo))
        .route("/datasets/{dataset_id}/export/yolo", get(export_yolo))
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

async fn export_yolo(
    State(state): State<AnnotationHttpState>,
    Path(dataset_id): Path<String>,
) -> Result<Json<YoloAnnotationExportResponse>, AnnotationRouteError> {
    let dataset_id =
        DatasetId::parse(dataset_id).map_err(|_| UseCaseError::Validation("invalid dataset id"))?;
    let export = ExportYoloAnnotationsUseCase::new(
        state.dataset_repository(),
        state.sample_repository(),
        state.annotation_repository(),
    )
    .execute(dataset_id)
    .await?;

    Ok(Json(mappers::yolo_export::yolo_annotation_export_response(
        export,
    )))
}

async fn import_yolo(
    State(state): State<AnnotationHttpState>,
    Path(dataset_id): Path<String>,
    Json(request): Json<YoloAnnotationImportRequest>,
) -> Result<(StatusCode, Json<YoloAnnotationImportResponse>), AnnotationRouteError> {
    let dataset_id =
        DatasetId::parse(dataset_id).map_err(|_| UseCaseError::Validation("invalid dataset id"))?;
    let use_case = ImportYoloAnnotationsUseCase::new(
        state.dataset_repository(),
        state.sample_repository(),
        state.annotation_repository(),
    );
    let result = use_case
        .execute(mappers::yolo_import::yolo_annotation_import_command(
            dataset_id, request,
        ))
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(mappers::yolo_import::yolo_annotation_import_response(
            result,
        )),
    ))
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
