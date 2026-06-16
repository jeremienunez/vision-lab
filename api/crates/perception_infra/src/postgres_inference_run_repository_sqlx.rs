use async_trait::async_trait;
use perception_app::{DetectionDraft, InferenceRunDraft, InferenceRunRepository, UseCaseError};
use perception_domain::{InferenceRunId, ModelId, NormalizedBbox};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{PgPool, Row, postgres::PgRow, types::Json};

pub struct PostgresInferenceRunRepository {
    pool: PgPool,
}

impl PostgresInferenceRunRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InferenceRunRepository for PostgresInferenceRunRepository {
    async fn create(&self, run: InferenceRunDraft) -> Result<InferenceRunDraft, UseCaseError> {
        sqlx::query(
            r#"
            INSERT INTO inference_runs (
                id, model_id, input_storage_uri, request_metadata, detections, latency_ms
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(run.id.into_uuid())
        .bind(run.model_id.into_uuid())
        .bind(&run.filename)
        .bind(Json(json!({
            "filename": &run.filename,
            "mime_type": &run.mime_type,
        })))
        .bind(Json(detections_to_json(&run.detections)?))
        .bind(
            i32::try_from(run.latency_ms)
                .map_err(|_| UseCaseError::Repository("postgres inference latency overflow"))?,
        )
        .execute(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres inference run create failed"))?;

        Ok(run)
    }

    async fn get(&self, run_id: InferenceRunId) -> Result<Option<InferenceRunDraft>, UseCaseError> {
        let row = sqlx::query(
            r#"
            SELECT id, model_id, input_storage_uri, request_metadata, detections, latency_ms
            FROM inference_runs
            WHERE id = $1
            "#,
        )
        .bind(run_id.into_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres inference run get failed"))?;

        row.map(row_to_inference_run).transpose()
    }
}

fn row_to_inference_run(row: PgRow) -> Result<InferenceRunDraft, UseCaseError> {
    let id: uuid::Uuid = row.get("id");
    let model_id: uuid::Uuid = row.get("model_id");
    let request_metadata: Json<Value> = row.get("request_metadata");
    let detections: Json<Value> = row.get("detections");
    let latency_ms: Option<i32> = row.get("latency_ms");

    Ok(InferenceRunDraft {
        id: InferenceRunId::parse(id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres inference run id invalid"))?,
        model_id: ModelId::parse(model_id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres inference run model id invalid"))?,
        filename: metadata_string(&request_metadata.0, "filename")?,
        mime_type: metadata_string(&request_metadata.0, "mime_type")?,
        latency_ms: u32::try_from(latency_ms.ok_or(UseCaseError::Repository(
            "postgres inference latency missing",
        ))?)
        .map_err(|_| UseCaseError::Repository("postgres inference latency invalid"))?,
        detections: detections_from_json(detections.0)?,
    })
}

fn metadata_string(value: &Value, field: &'static str) -> Result<String, UseCaseError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or(UseCaseError::Repository(
            "postgres inference request metadata invalid",
        ))
}

fn detections_to_json(detections: &[DetectionDraft]) -> Result<Value, UseCaseError> {
    serde_json::to_value(
        detections
            .iter()
            .map(StoredDetection::from_detection)
            .collect::<Vec<_>>(),
    )
    .map_err(|_| UseCaseError::Repository("postgres inference detections encode failed"))
}

fn detections_from_json(value: Value) -> Result<Vec<DetectionDraft>, UseCaseError> {
    let detections = serde_json::from_value::<Vec<StoredDetection>>(value)
        .map_err(|_| UseCaseError::Repository("postgres inference detections invalid"))?;

    detections
        .into_iter()
        .map(StoredDetection::into_detection)
        .collect()
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredDetection {
    class_id: u32,
    class_name: String,
    confidence: f32,
    bbox: StoredBbox,
    distance_m: Option<f32>,
}

impl StoredDetection {
    fn from_detection(detection: &DetectionDraft) -> Self {
        Self {
            class_id: detection.class_id,
            class_name: detection.class_name.clone(),
            confidence: detection.confidence,
            bbox: StoredBbox::from_bbox(detection.bbox),
            distance_m: detection.distance_m,
        }
    }

    fn into_detection(self) -> Result<DetectionDraft, UseCaseError> {
        Ok(DetectionDraft {
            class_id: self.class_id,
            class_name: self.class_name,
            confidence: self.confidence,
            bbox: self.bbox.into_bbox()?,
            distance_m: self.distance_m,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredBbox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl StoredBbox {
    fn from_bbox(bbox: NormalizedBbox) -> Self {
        Self {
            x: bbox.x,
            y: bbox.y,
            width: bbox.width,
            height: bbox.height,
        }
    }

    fn into_bbox(self) -> Result<NormalizedBbox, UseCaseError> {
        NormalizedBbox::new(self.x, self.y, self.width, self.height)
            .map_err(|_| UseCaseError::Repository("postgres inference detection bbox invalid"))
    }
}
