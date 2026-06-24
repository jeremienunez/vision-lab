import { useEffect, useMemo, useState } from 'react';
import { GitBranch, Play, RefreshCw } from 'lucide-react';

import { createPerceptionApi } from '../../perception-api.js';
import { orderCameraModels } from '../../camera-models.js';
import { useConfigContext } from '../../context/ConfigContext.jsx';
import { usePerceptionDataContext } from '../../context/PerceptionDataContext.jsx';
import { Panel } from '../../components/Panel.jsx';
import { SegmentedControl } from '../../components/SegmentedControl.jsx';
import { StateChip } from '../../components/StateChip.jsx';
import { EmptyState } from '../../components/EmptyState.jsx';
import { STATUS_DOT } from '../../components/tone.js';
import { Field } from '../../components/Field.jsx';
import { Select } from '../../components/Select.jsx';
import { TextInput } from '../../components/TextInput.jsx';
import { Button } from '../../components/Button.jsx';
import {
  DEFAULT_TRAINING_FORM,
  buildDatasetVersionOptions,
  buildTrainingJobCommand,
  preferredDatasetVersionId,
  preferredBaseModelPath,
} from './training-form.js';

const STATUS_ORDER = ['queued', 'running', 'succeeded', 'failed', 'cancelled'];

export function TrainingPage() {
  const { config } = useConfigContext();
  const { payload, viewModel, refresh } = usePerceptionDataContext();
  const [filter, setFilter] = useState('all');
  const [versionsByDatasetId, setVersionsByDatasetId] = useState({});
  const [versionsLoading, setVersionsLoading] = useState(false);
  const [versionsError, setVersionsError] = useState('');
  const [form, setForm] = useState(DEFAULT_TRAINING_FORM);
  const [submitting, setSubmitting] = useState(false);
  const [submitError, setSubmitError] = useState('');
  const [createdJob, setCreatedJob] = useState(null);
  const datasets = payload.datasets ?? [];
  const jobs = payload.trainingJobs ?? [];
  const models = useMemo(() => orderCameraModels(payload.models ?? []), [payload.models]);
  const datasetIdsKey = useMemo(() => datasets.map((dataset) => dataset.id).join('|'), [datasets]);

  const options = useMemo(
    () => [
      { value: 'all', label: 'all' },
      ...STATUS_ORDER.filter((status) => viewModel.jobStatusCounts[status]).map((status) => ({
        value: status,
        label: status,
      })),
    ],
    [viewModel.jobStatusCounts],
  );

  const visibleJobs = filter === 'all' ? jobs : jobs.filter((job) => job.status === filter);
  const versionOptions = useMemo(
    () => buildDatasetVersionOptions(datasets, versionsByDatasetId),
    [datasets, versionsByDatasetId],
  );
  const versionOptionsKey = useMemo(
    () => versionOptions.map((option) => option.value).join('|'),
    [versionOptions],
  );
  const defaultBaseModel = useMemo(() => preferredBaseModelPath(models), [models]);
  const selectedVersion = versionOptions.find((option) => option.value === form.datasetVersionId);

  useEffect(() => {
    let cancelled = false;

    async function loadVersions() {
      if (!datasetIdsKey) {
        setVersionsByDatasetId({});
        return;
      }

      setVersionsLoading(true);
      setVersionsError('');

      try {
        const api = createPerceptionApi(config);
        const entries = await Promise.all(
          datasets.map(async (dataset) => [dataset.id, await api.listDatasetVersions(dataset.id)]),
        );
        if (!cancelled) setVersionsByDatasetId(Object.fromEntries(entries));
      } catch (error) {
        if (!cancelled) {
          setVersionsByDatasetId({});
          setVersionsError(error.message);
        }
      } finally {
        if (!cancelled) setVersionsLoading(false);
      }
    }

    loadVersions();

    return () => {
      cancelled = true;
    };
  }, [config, datasets, datasetIdsKey]);

  useEffect(() => {
    const preferredVersionId = preferredDatasetVersionId(models, versionOptions);

    setForm((current) => ({
      ...current,
      datasetVersionId: current.datasetVersionId || preferredVersionId,
      baseModel: current.baseModel || defaultBaseModel,
    }));
  }, [defaultBaseModel, models, versionOptions, versionOptionsKey]);

  function updateForm(field, value) {
    setForm((current) => ({ ...current, [field]: value }));
  }

  async function submitTrainingJob(event) {
    event.preventDefault();
    setSubmitting(true);
    setSubmitError('');
    setCreatedJob(null);

    try {
      const api = createPerceptionApi(config);
      const job = await api.createTrainingJob(buildTrainingJobCommand(form));
      setCreatedJob(job);
      await refresh();
    } catch (error) {
      setSubmitError(error.message);
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="grid min-w-0 gap-6 xl:grid-cols-[minmax(0,0.85fr)_minmax(0,1.15fr)]">
      <Panel id="start-training" title="Start training" action={versionsLoading ? 'loading versions' : 'queue job'} icon={Play}>
        <form className="flex min-w-0 flex-col gap-4" onSubmit={submitTrainingJob}>
          <Field label="Dataset version">
            <Select
              value={form.datasetVersionId}
              onChange={(event) => updateForm('datasetVersionId', event.target.value)}
              disabled={versionsLoading || versionOptions.length === 0}
              required
            >
              {versionOptions.length === 0 ? (
                <option value="">No version available</option>
              ) : (
                versionOptions.map((option) => (
                  <option key={option.value} value={option.value}>
                    {option.label}
                  </option>
                ))
              )}
            </Select>
          </Field>

          {selectedVersion && (
            <div className="rounded-lg border border-line bg-surface-soft px-3 py-2 text-xs text-muted">
              {selectedVersion.detail}
            </div>
          )}

          <Field label="Model family">
            <TextInput
              value={form.modelFamily}
              onChange={(event) => updateForm('modelFamily', event.target.value)}
              required
            />
          </Field>

          <Field label="Base model">
            <TextInput
              value={form.baseModel}
              onChange={(event) => updateForm('baseModel', event.target.value)}
              list="training-base-models"
              placeholder=".perceptionlab/models/yolo11n.pt"
            />
          </Field>
          <datalist id="training-base-models">
            {models
              .filter((model) => model.artifact_uri)
              .map((model) => (
                <option key={model.id} value={model.artifact_uri.replace(/^file:\/\//, '')}>
                  {model.name}
                </option>
              ))}
          </datalist>

          <div className="grid grid-cols-2 gap-3">
            <Field label="Epochs">
              <TextInput
                type="number"
                min="1"
                value={form.epochs}
                onChange={(event) => updateForm('epochs', event.target.value)}
                required
              />
            </Field>
            <Field label="Batch">
              <TextInput
                type="number"
                min="1"
                value={form.batchSize}
                onChange={(event) => updateForm('batchSize', event.target.value)}
                required
              />
            </Field>
            <Field label="Image size">
              <TextInput
                type="number"
                min="64"
                step="32"
                value={form.imageSize}
                onChange={(event) => updateForm('imageSize', event.target.value)}
                required
              />
            </Field>
            <Field label="Learning rate">
              <TextInput
                type="number"
                min="0.000001"
                step="any"
                value={form.learningRate}
                onChange={(event) => updateForm('learningRate', event.target.value)}
                required
              />
            </Field>
          </div>

          {versionsError && <p className="text-sm text-danger">{versionsError}</p>}
          {submitError && <p className="text-sm text-danger">{submitError}</p>}
          {createdJob && (
            <div className="flex items-center justify-between gap-3 rounded-lg border border-line bg-surface-soft px-3 py-2 text-sm">
              <span className="truncate text-muted">{createdJob.id}</span>
              <StateChip status={createdJob.status} />
            </div>
          )}

          <div className="flex items-center justify-between gap-3">
            <Button type="button" icon={RefreshCw} onClick={() => refresh()} disabled={submitting}>
              Refresh
            </Button>
            <Button
              type="submit"
              variant="primary"
              icon={Play}
              disabled={submitting || versionsLoading || versionOptions.length === 0}
            >
              {submitting ? 'Starting' : 'Start training'}
            </Button>
          </div>
        </form>
      </Panel>

      <Panel id="training" title="Training queue" action={`${viewModel.activeJobCount} active`} icon={GitBranch}>
        <div className="mb-4">
          <SegmentedControl options={options} selected={filter} onSelect={setFilter} ariaLabel="Training job filter" />
        </div>

        {visibleJobs.length === 0 ? (
          <EmptyState icon={GitBranch} label="No training jobs" />
        ) : (
          <div className="flex flex-col gap-2">
            {visibleJobs.map((job) => (
              <article
                key={job.id}
                className="flex items-center gap-3 rounded-xl border border-line bg-surface-soft px-4 py-3"
              >
                <span
                  className={`h-2.5 w-2.5 shrink-0 rounded-full ${STATUS_DOT[job.status] ?? 'bg-subtle'}`}
                  aria-hidden="true"
                />
                <div className="flex min-w-0 flex-1 flex-col">
                  <strong className="truncate text-ink">{job.model_family}</strong>
                  <small className="truncate text-subtle">{job.dataset_version_id}</small>
                </div>
                <StateChip status={job.status} />
              </article>
            ))}
          </div>
        )}
      </Panel>
    </div>
  );
}
