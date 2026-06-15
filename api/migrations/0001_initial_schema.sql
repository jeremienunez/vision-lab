BEGIN;

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE datasets (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL,
  description text,
  task_type text NOT NULL,
  classes jsonb NOT NULL DEFAULT '[]'::jsonb,
  status text NOT NULL DEFAULT 'draft',
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT datasets_name_key UNIQUE (name),
  CONSTRAINT dataset_status_check CHECK (status IN ('draft', 'ready', 'archived')),
  CONSTRAINT dataset_task_type_check CHECK (task_type IN ('object_detection')),
  CONSTRAINT dataset_classes_json_check CHECK (jsonb_typeof(classes) = 'array')
);

CREATE TABLE dataset_classes (
  dataset_id uuid NOT NULL REFERENCES datasets(id) ON DELETE CASCADE,
  class_id integer NOT NULL,
  class_name text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (dataset_id, class_id),
  CONSTRAINT dataset_class_name_key UNIQUE (dataset_id, class_name),
  CONSTRAINT dataset_class_identity_key UNIQUE (dataset_id, class_id, class_name),
  CONSTRAINT dataset_class_id_check CHECK (class_id >= 0),
  CONSTRAINT dataset_class_name_check CHECK (length(trim(class_name)) > 0)
);

CREATE TABLE samples (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  dataset_id uuid NOT NULL REFERENCES datasets(id) ON DELETE CASCADE,
  storage_uri text NOT NULL,
  filename text NOT NULL,
  mime_type text NOT NULL,
  width integer NOT NULL,
  height integer NOT NULL,
  size_bytes bigint NOT NULL,
  checksum text NOT NULL,
  source text NOT NULL DEFAULT 'upload',
  metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
  created_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT sample_dataset_identity_key UNIQUE (id, dataset_id),
  CONSTRAINT sample_dataset_checksum_key UNIQUE (dataset_id, checksum),
  CONSTRAINT sample_dimensions_check CHECK (width > 0 AND height > 0),
  CONSTRAINT sample_size_bytes_check CHECK (size_bytes >= 0),
  CONSTRAINT sample_metadata_json_check CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE TABLE annotations (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  sample_id uuid NOT NULL,
  dataset_id uuid NOT NULL,
  class_id integer NOT NULL,
  class_name text NOT NULL,
  bbox_x numeric(8, 7) NOT NULL,
  bbox_y numeric(8, 7) NOT NULL,
  bbox_width numeric(8, 7) NOT NULL,
  bbox_height numeric(8, 7) NOT NULL,
  format text NOT NULL DEFAULT 'normalized_xywh',
  confidence numeric(5, 4),
  source text NOT NULL DEFAULT 'manual',
  created_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT annotation_dataset_identity_key UNIQUE (id, dataset_id),
  CONSTRAINT annotation_sample_fk FOREIGN KEY (sample_id, dataset_id)
    REFERENCES samples(id, dataset_id) ON DELETE CASCADE,
  CONSTRAINT annotation_class_fk FOREIGN KEY (dataset_id, class_id, class_name)
    REFERENCES dataset_classes(dataset_id, class_id, class_name) ON DELETE RESTRICT,
  CONSTRAINT annotation_bbox_bounds_check CHECK (
    bbox_x >= 0
    AND bbox_y >= 0
    AND bbox_width > 0
    AND bbox_height > 0
    AND bbox_x + bbox_width <= 1
    AND bbox_y + bbox_height <= 1
  ),
  CONSTRAINT annotation_format_check CHECK (format IN ('normalized_xywh')),
  CONSTRAINT annotation_confidence_check CHECK (confidence IS NULL OR confidence BETWEEN 0 AND 1)
);

CREATE TABLE dataset_versions (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  dataset_id uuid NOT NULL REFERENCES datasets(id) ON DELETE RESTRICT,
  version_name text NOT NULL,
  sample_count integer NOT NULL,
  annotation_count integer NOT NULL,
  classes_snapshot jsonb NOT NULL,
  split_config jsonb NOT NULL DEFAULT '{}'::jsonb,
  created_at timestamptz NOT NULL DEFAULT now(),
  created_by text NOT NULL DEFAULT 'system',
  CONSTRAINT dataset_version_dataset_identity_key UNIQUE (id, dataset_id),
  CONSTRAINT dataset_version_name_key UNIQUE (dataset_id, version_name),
  CONSTRAINT dataset_version_counts_check CHECK (sample_count >= 0 AND annotation_count >= 0),
  CONSTRAINT dataset_version_classes_json_check CHECK (jsonb_typeof(classes_snapshot) = 'array'),
  CONSTRAINT dataset_version_split_json_check CHECK (jsonb_typeof(split_config) = 'object')
);

CREATE TABLE dataset_version_samples (
  dataset_version_id uuid NOT NULL,
  dataset_id uuid NOT NULL,
  sample_id uuid NOT NULL,
  split_name text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (dataset_version_id, sample_id),
  CONSTRAINT dataset_version_sample_version_fk FOREIGN KEY (dataset_version_id, dataset_id)
    REFERENCES dataset_versions(id, dataset_id) ON DELETE CASCADE,
  CONSTRAINT dataset_version_sample_sample_fk FOREIGN KEY (sample_id, dataset_id)
    REFERENCES samples(id, dataset_id) ON DELETE RESTRICT,
  CONSTRAINT dataset_version_sample_split_check CHECK (split_name IN ('train', 'validation', 'test'))
);

CREATE TABLE dataset_version_annotations (
  dataset_version_id uuid NOT NULL,
  dataset_id uuid NOT NULL,
  annotation_id uuid NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (dataset_version_id, annotation_id),
  CONSTRAINT dataset_version_annotation_version_fk FOREIGN KEY (dataset_version_id, dataset_id)
    REFERENCES dataset_versions(id, dataset_id) ON DELETE CASCADE,
  CONSTRAINT dataset_version_annotation_annotation_fk FOREIGN KEY (annotation_id, dataset_id)
    REFERENCES annotations(id, dataset_id) ON DELETE RESTRICT
);

CREATE FUNCTION dataset_version_immutable() RETURNS trigger AS $$
BEGIN
  RAISE EXCEPTION 'Dataset versions are immutable after creation.';
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER dataset_versions_are_immutable
BEFORE UPDATE OR DELETE ON dataset_versions
FOR EACH ROW EXECUTE FUNCTION dataset_version_immutable();

CREATE TRIGGER dataset_version_samples_are_immutable
BEFORE UPDATE OR DELETE ON dataset_version_samples
FOR EACH ROW EXECUTE FUNCTION dataset_version_immutable();

CREATE TRIGGER dataset_version_annotations_are_immutable
BEFORE UPDATE OR DELETE ON dataset_version_annotations
FOR EACH ROW EXECUTE FUNCTION dataset_version_immutable();

CREATE TABLE training_jobs (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  dataset_version_id uuid NOT NULL REFERENCES dataset_versions(id) ON DELETE RESTRICT,
  model_family text NOT NULL,
  base_model text,
  status text NOT NULL DEFAULT 'queued',
  hyperparameters jsonb NOT NULL DEFAULT '{}'::jsonb,
  created_at timestamptz NOT NULL DEFAULT now(),
  started_at timestamptz,
  finished_at timestamptz,
  error_message text,
  output_model_id uuid,
  CONSTRAINT training_job_status_check CHECK (
    status IN ('queued', 'running', 'succeeded', 'failed', 'cancelled')
  ),
  CONSTRAINT training_job_hyperparameters_json_check CHECK (jsonb_typeof(hyperparameters) = 'object'),
  CONSTRAINT training_job_error_message_check CHECK (
    status <> 'failed' OR length(coalesce(error_message, '')) > 0
  ),
  CONSTRAINT training_job_timestamps_check CHECK (
    started_at IS NULL
    OR finished_at IS NULL
    OR finished_at >= started_at
  )
);

CREATE TABLE training_job_queue (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  training_job_id uuid NOT NULL REFERENCES training_jobs(id) ON DELETE CASCADE,
  status text NOT NULL DEFAULT 'queued',
  available_at timestamptz NOT NULL DEFAULT now(),
  leased_until timestamptz,
  locked_by text,
  attempts integer NOT NULL DEFAULT 0,
  last_error text,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT training_job_queue_job_key UNIQUE (training_job_id),
  CONSTRAINT training_job_queue_status_check CHECK (
    status IN ('queued', 'leased', 'completed', 'failed', 'cancelled')
  ),
  CONSTRAINT training_job_queue_attempts_check CHECK (attempts >= 0),
  CONSTRAINT training_job_queue_lease_check CHECK (
    status <> 'leased' OR leased_until IS NOT NULL
  )
);

CREATE TABLE job_events (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  training_job_id uuid NOT NULL REFERENCES training_jobs(id) ON DELETE CASCADE,
  event_type text NOT NULL,
  payload jsonb NOT NULL DEFAULT '{}'::jsonb,
  created_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT job_event_type_check CHECK (length(trim(event_type)) > 0),
  CONSTRAINT job_event_payload_json_check CHECK (jsonb_typeof(payload) = 'object')
);

CREATE TABLE training_metrics (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  training_job_id uuid NOT NULL REFERENCES training_jobs(id) ON DELETE CASCADE,
  split_name text NOT NULL,
  metric_name text NOT NULL,
  metric_value double precision NOT NULL,
  step integer,
  epoch integer,
  metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
  created_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT training_metric_split_check CHECK (split_name IN ('train', 'validation', 'test')),
  CONSTRAINT training_metric_name_check CHECK (length(trim(metric_name)) > 0),
  CONSTRAINT training_metric_step_check CHECK (step IS NULL OR step >= 0),
  CONSTRAINT training_metric_epoch_check CHECK (epoch IS NULL OR epoch >= 0),
  CONSTRAINT training_metric_metadata_json_check CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE TABLE models (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL,
  version text NOT NULL,
  training_job_id uuid NOT NULL REFERENCES training_jobs(id) ON DELETE RESTRICT,
  dataset_version_id uuid NOT NULL REFERENCES dataset_versions(id) ON DELETE RESTRICT,
  model_family text NOT NULL,
  artifact_uri text NOT NULL,
  metrics_summary jsonb NOT NULL DEFAULT '{}'::jsonb,
  status text NOT NULL DEFAULT 'candidate',
  created_at timestamptz NOT NULL DEFAULT now(),
  promoted_at timestamptz,
  CONSTRAINT model_name_version_key UNIQUE (name, version),
  CONSTRAINT model_training_job_key UNIQUE (training_job_id),
  CONSTRAINT model_status_check CHECK (status IN ('candidate', 'validated', 'promoted', 'archived')),
  CONSTRAINT model_metrics_summary_json_check CHECK (jsonb_typeof(metrics_summary) = 'object')
);

ALTER TABLE training_jobs
ADD CONSTRAINT training_job_output_model_fk
FOREIGN KEY (output_model_id) REFERENCES models(id) ON DELETE SET NULL;

CREATE TABLE model_exports (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  model_id uuid NOT NULL REFERENCES models(id) ON DELETE CASCADE,
  format text NOT NULL,
  artifact_uri text,
  status text NOT NULL DEFAULT 'queued',
  created_at timestamptz NOT NULL DEFAULT now(),
  finished_at timestamptz,
  error_message text,
  CONSTRAINT export_status_check CHECK (status IN ('queued', 'running', 'succeeded', 'failed')),
  CONSTRAINT export_format_check CHECK (format IN ('pt', 'torchscript', 'onnx', 'coreml')),
  CONSTRAINT export_error_message_check CHECK (
    status <> 'failed' OR length(coalesce(error_message, '')) > 0
  )
);

CREATE TABLE inference_runs (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  model_id uuid NOT NULL REFERENCES models(id) ON DELETE RESTRICT,
  input_storage_uri text NOT NULL,
  output_storage_uri text,
  request_metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
  detections jsonb NOT NULL DEFAULT '[]'::jsonb,
  latency_ms integer,
  created_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT inference_request_metadata_json_check CHECK (jsonb_typeof(request_metadata) = 'object'),
  CONSTRAINT inference_detections_json_check CHECK (jsonb_typeof(detections) = 'array'),
  CONSTRAINT inference_latency_check CHECK (latency_ms IS NULL OR latency_ms >= 0)
);

CREATE TABLE artifacts (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  artifact_uri text NOT NULL,
  artifact_type text NOT NULL,
  owner_type text NOT NULL,
  owner_id uuid NOT NULL,
  checksum text,
  size_bytes bigint,
  metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
  created_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT artifact_uri_key UNIQUE (artifact_uri),
  CONSTRAINT artifact_owner_type_check CHECK (
    owner_type IN (
      'dataset_sample',
      'dataset_version',
      'training_job',
      'model',
      'model_export',
      'inference_run',
      'other'
    )
  ),
  CONSTRAINT artifact_size_bytes_check CHECK (size_bytes IS NULL OR size_bytes >= 0),
  CONSTRAINT artifact_metadata_json_check CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE INDEX datasets_status_idx ON datasets(status);
CREATE INDEX dataset_classes_dataset_idx ON dataset_classes(dataset_id);
CREATE INDEX samples_dataset_idx ON samples(dataset_id);
CREATE INDEX annotations_sample_idx ON annotations(sample_id);
CREATE INDEX annotations_dataset_class_idx ON annotations(dataset_id, class_id);
CREATE INDEX dataset_versions_dataset_idx ON dataset_versions(dataset_id);
CREATE INDEX dataset_version_samples_sample_idx ON dataset_version_samples(sample_id);
CREATE INDEX dataset_version_annotations_annotation_idx ON dataset_version_annotations(annotation_id);
CREATE INDEX training_jobs_dataset_version_idx ON training_jobs(dataset_version_id);
CREATE INDEX training_jobs_status_idx ON training_jobs(status);
CREATE INDEX training_job_queue_ready_idx ON training_job_queue(status, available_at);
CREATE INDEX job_events_training_job_idx ON job_events(training_job_id, created_at);
CREATE INDEX training_metrics_job_idx ON training_metrics(training_job_id, split_name, metric_name);
CREATE INDEX models_dataset_version_idx ON models(dataset_version_id);
CREATE INDEX models_status_idx ON models(status);
CREATE INDEX model_exports_model_idx ON model_exports(model_id);
CREATE INDEX inference_runs_model_idx ON inference_runs(model_id, created_at);
CREATE INDEX artifacts_owner_idx ON artifacts(owner_type, owner_id);

COMMIT;
