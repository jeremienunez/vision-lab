use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use perception_app::{InferenceEngine, InferenceRequest, ModelDraft, UseCaseError};
use perception_domain::{DatasetVersionId, ModelId, ModelStatus, TrainingJobId};
use perception_infra::{
    YoloCliCommand, YoloCliCommandOutput, YoloCliCommandRunner, YoloCliInferenceConfig,
    YoloCliInferenceEngine,
};

#[derive(Default)]
struct RecordingRunner {
    commands: Mutex<Vec<YoloCliCommand>>,
}

impl YoloCliCommandRunner for RecordingRunner {
    fn run(&self, command: YoloCliCommand) -> Result<YoloCliCommandOutput, UseCaseError> {
        self.commands
            .lock()
            .expect("commands mutex is available")
            .push(command);
        Ok(YoloCliCommandOutput {
            success: true,
            stdout: r#"{
  "image_width": 200,
  "image_height": 400,
  "detections": [
    {
      "class_id": 0,
      "class_name": "person",
      "confidence": 0.87,
      "bbox_xyxy": [20.0, 40.0, 120.0, 240.0]
    }
  ]
}"#
            .to_owned(),
            stderr: String::new(),
        })
    }
}

fn model_fixture(project_root: &str) -> ModelDraft {
    ModelDraft {
        id: ModelId::new(),
        name: "yolo-real".to_owned(),
        version: "v1".to_owned(),
        training_job_id: TrainingJobId::new(),
        dataset_version_id: DatasetVersionId::new(),
        model_family: "yolo".to_owned(),
        artifact_uri: format!("file://{project_root}/.perceptionlab/models/yolo11n.pt"),
        metrics_summary: Default::default(),
        status: ModelStatus::Candidate,
    }
}

#[tokio::test]
async fn yolo_cli_inference_engine_runs_worker_cli_and_maps_detections() {
    let temp_root = std::env::temp_dir().join(format!("perceptionlab-yolo-cli-{}", ModelId::new()));
    let project_root = temp_root.join("repo");
    let runner = Arc::new(RecordingRunner::default());
    let engine = YoloCliInferenceEngine::new(
        YoloCliInferenceConfig {
            project_root: project_root.clone(),
            worker_project: PathBuf::from("worker"),
            uv_cache_dir: PathBuf::from(".perceptionlab/cache/uv"),
            output_root: PathBuf::from(".perceptionlab/real-inference/api"),
            temp_root: temp_root.join("tmp"),
            uv_binary: "uv".to_owned(),
        },
        runner.clone(),
    );

    let result = engine
        .infer(InferenceRequest {
            model: model_fixture(project_root.to_str().expect("project root is utf8")),
            filename: "feet.png".to_owned(),
            mime_type: "image/png".to_owned(),
            image_bytes: vec![137, 80, 78, 71],
            confidence_threshold: 0.40,
        })
        .await
        .expect("inference succeeds");

    assert_eq!(result.detections.len(), 1);
    assert_eq!(result.detections[0].class_name, "person");
    assert_eq!(result.detections[0].confidence, 0.87);
    assert_eq!(result.detections[0].bbox.x, 0.10);
    assert_eq!(result.detections[0].bbox.y, 0.10);
    assert_eq!(result.detections[0].bbox.width, 0.50);
    assert_eq!(result.detections[0].bbox.height, 0.50);

    let commands = runner.commands.lock().expect("commands mutex is available");
    assert_eq!(commands.len(), 1);
    let command = &commands[0];
    assert_eq!(command.program, "uv");
    assert_eq!(command.current_dir, project_root);
    assert_eq!(
        command.env.get("UV_CACHE_DIR").map(String::as_str),
        Some(".perceptionlab/cache/uv")
    );
    assert!(command.args.contains(&"--json-only".to_owned()));
    assert!(command.args.contains(&"detect-image".to_owned()));
    assert!(
        command
            .args
            .windows(2)
            .any(|window| window[0] == "--confidence-threshold" && window[1] == "0.4")
    );
    assert!(
        command
            .args
            .windows(2)
            .any(|window| window[0] == "--model-path"
                && window[1].ends_with(".perceptionlab/models/yolo11n.pt"))
    );
}
