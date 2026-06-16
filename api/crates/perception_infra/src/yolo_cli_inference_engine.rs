use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    time::Instant,
};

use async_trait::async_trait;
use perception_app::{
    DetectionDraft, InferenceEngine, InferenceRequest, InferenceResult, UseCaseError,
};
use perception_domain::{InferenceRunId, NormalizedBbox};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YoloCliInferenceConfig {
    pub project_root: PathBuf,
    pub worker_project: PathBuf,
    pub uv_cache_dir: PathBuf,
    pub output_root: PathBuf,
    pub temp_root: PathBuf,
    pub uv_binary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YoloCliCommand {
    pub program: String,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub current_dir: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YoloCliCommandOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

pub trait YoloCliCommandRunner: Send + Sync {
    fn run(&self, command: YoloCliCommand) -> Result<YoloCliCommandOutput, UseCaseError>;
}

pub struct StdYoloCliCommandRunner;

impl YoloCliCommandRunner for StdYoloCliCommandRunner {
    fn run(&self, command: YoloCliCommand) -> Result<YoloCliCommandOutput, UseCaseError> {
        let output = Command::new(&command.program)
            .args(&command.args)
            .envs(&command.env)
            .current_dir(&command.current_dir)
            .output()
            .map_err(|_| UseCaseError::Repository("yolo cli inference failed"))?;

        Ok(YoloCliCommandOutput {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

pub struct YoloCliInferenceEngine {
    config: YoloCliInferenceConfig,
    runner: Arc<dyn YoloCliCommandRunner>,
}

impl YoloCliInferenceEngine {
    pub fn new(config: YoloCliInferenceConfig, runner: Arc<dyn YoloCliCommandRunner>) -> Self {
        Self { config, runner }
    }

    pub fn from_env() -> Self {
        let project_root = std::env::var("PERCEPTIONLAB_PROJECT_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        let config = YoloCliInferenceConfig {
            project_root,
            worker_project: std::env::var("PERCEPTIONLAB_WORKER_PROJECT")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("worker")),
            uv_cache_dir: std::env::var("PERCEPTIONLAB_UV_CACHE_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from(".perceptionlab/cache/uv")),
            output_root: std::env::var("PERCEPTIONLAB_REAL_INFERENCE_OUTPUT_ROOT")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from(".perceptionlab/real-inference/api")),
            temp_root: std::env::var("PERCEPTIONLAB_TMP_ROOT")
                .map(|root| PathBuf::from(root).join("inference"))
                .unwrap_or_else(|_| PathBuf::from(".perceptionlab/tmp/inference")),
            uv_binary: std::env::var("PERCEPTIONLAB_UV_BINARY").unwrap_or_else(|_| "uv".to_owned()),
        };

        Self::new(config, Arc::new(StdYoloCliCommandRunner))
    }
}

#[async_trait]
impl InferenceEngine for YoloCliInferenceEngine {
    async fn infer(&self, request: InferenceRequest) -> Result<InferenceResult, UseCaseError> {
        let start = Instant::now();
        std::fs::create_dir_all(&self.config.temp_root)
            .map_err(|_| UseCaseError::Repository("yolo cli inference temp write failed"))?;
        let input_path = self.config.temp_root.join(format!(
            "{}.{}",
            InferenceRunId::new(),
            image_extension(&request)
        ));
        std::fs::write(&input_path, &request.image_bytes)
            .map_err(|_| UseCaseError::Repository("yolo cli inference temp write failed"))?;

        let run_name = format!("api-{}", InferenceRunId::new());
        let command = self.command_for(&request, &input_path, &run_name);
        let output = self.runner.run(command)?;
        if !output.success {
            return Err(UseCaseError::Repository("yolo cli inference failed"));
        }
        let summary = parse_summary(&output.stdout)?;

        Ok(InferenceResult {
            run_id: InferenceRunId::new(),
            model_id: request.model.id,
            latency_ms: start.elapsed().as_millis().try_into().unwrap_or(u32::MAX),
            detections: summary.into_detections()?,
        })
    }
}

impl YoloCliInferenceEngine {
    fn command_for(
        &self,
        request: &InferenceRequest,
        input_path: &Path,
        run_name: &str,
    ) -> YoloCliCommand {
        let mut env = BTreeMap::new();
        env.insert(
            "UV_CACHE_DIR".to_owned(),
            self.config.uv_cache_dir.to_string_lossy().into_owned(),
        );
        YoloCliCommand {
            program: self.config.uv_binary.clone(),
            args: vec![
                "run".to_owned(),
                "--project".to_owned(),
                self.config.worker_project.to_string_lossy().into_owned(),
                "perception-worker".to_owned(),
                "detect-image".to_owned(),
                input_path.to_string_lossy().into_owned(),
                "--model-path".to_owned(),
                model_path_from_artifact_uri(&request.model.artifact_uri),
                "--output-root".to_owned(),
                self.config.output_root.to_string_lossy().into_owned(),
                "--run-name".to_owned(),
                run_name.to_owned(),
                "--confidence-threshold".to_owned(),
                request.confidence_threshold.to_string(),
                "--json-only".to_owned(),
            ],
            env,
            current_dir: self.config.project_root.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct YoloCliSummary {
    image_width: u32,
    image_height: u32,
    detections: Vec<YoloCliDetection>,
}

#[derive(Debug, Deserialize)]
struct YoloCliDetection {
    class_id: u32,
    class_name: String,
    confidence: f32,
    bbox_xyxy: [f32; 4],
}

impl YoloCliSummary {
    fn into_detections(self) -> Result<Vec<DetectionDraft>, UseCaseError> {
        if self.image_width == 0 || self.image_height == 0 {
            return Err(UseCaseError::Repository(
                "yolo cli inference invalid image size",
            ));
        }
        self.detections
            .into_iter()
            .map(|detection| detection.into_detection(self.image_width, self.image_height))
            .collect()
    }
}

impl YoloCliDetection {
    fn into_detection(
        self,
        image_width: u32,
        image_height: u32,
    ) -> Result<DetectionDraft, UseCaseError> {
        let [x1, y1, x2, y2] = self.bbox_xyxy;
        let image_width = image_width as f32;
        let image_height = image_height as f32;
        let x1 = x1.clamp(0.0, image_width);
        let y1 = y1.clamp(0.0, image_height);
        let x2 = x2.clamp(x1, image_width);
        let y2 = y2.clamp(y1, image_height);
        let x = x1 / image_width;
        let y = y1 / image_height;
        let width = (x2 - x1) / image_width;
        let height = (y2 - y1) / image_height;

        Ok(DetectionDraft {
            class_id: self.class_id,
            class_name: self.class_name,
            confidence: self.confidence,
            bbox: NormalizedBbox::new(x, y, width, height)
                .map_err(|_| UseCaseError::Repository("yolo cli inference invalid bbox"))?,
            distance_m: None,
        })
    }
}

fn parse_summary(stdout: &str) -> Result<YoloCliSummary, UseCaseError> {
    serde_json::from_str(stdout.trim())
        .map_err(|_| UseCaseError::Repository("yolo cli inference invalid output"))
}

fn model_path_from_artifact_uri(artifact_uri: &str) -> String {
    artifact_uri
        .strip_prefix("file://")
        .unwrap_or(artifact_uri)
        .to_owned()
}

fn image_extension(request: &InferenceRequest) -> String {
    match request.mime_type.as_str() {
        "image/jpeg" | "image/jpg" => "jpg".to_owned(),
        "image/png" => "png".to_owned(),
        "image/webp" => "webp".to_owned(),
        _ => request
            .filename
            .rsplit_once('.')
            .map(|(_, extension)| extension)
            .filter(|extension| ["jpg", "jpeg", "png", "webp"].contains(extension))
            .unwrap_or("png")
            .to_owned(),
    }
}
