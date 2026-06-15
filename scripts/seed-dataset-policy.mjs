import fs from 'node:fs';
import path from 'node:path';

const requiredClasses = ['cup', 'book', 'phone', 'keyboard', 'mouse'];

export function loadSeedManifest(seedRoot = 'datasets/seed') {
  return JSON.parse(fs.readFileSync(path.join(seedRoot, 'manifest.json'), 'utf8'));
}

export function validateSeedDataset(seedRoot = 'datasets/seed') {
  const errors = [];
  const manifestPath = path.join(seedRoot, 'manifest.json');

  if (!fs.existsSync(manifestPath)) {
    return [`Missing seed manifest: ${manifestPath}.`];
  }

  const manifest = loadSeedManifest(seedRoot);
  if (manifest.dataset?.name !== 'desk-objects-v1') {
    errors.push('Seed dataset name must be desk-objects-v1.');
  }

  for (const className of requiredClasses) {
    if (!manifest.dataset?.classes?.includes(className)) {
      errors.push(`Seed dataset is missing class ${className}.`);
    }
  }

  if (!Array.isArray(manifest.samples) || manifest.samples.length === 0) {
    errors.push('Seed dataset must define at least one sample.');
  }

  for (const sample of manifest.samples ?? []) {
    if (!sample.path || !fs.existsSync(path.join(seedRoot, sample.path))) {
      errors.push(`Seed sample file is missing: ${sample.path}.`);
    }

    if (!sample.source_svg || !fs.existsSync(path.join(seedRoot, sample.source_svg))) {
      errors.push(`Seed sample source SVG is missing: ${sample.source_svg}.`);
    }

    if (sample.mime_type !== 'image/png') {
      errors.push(`Seed sample ${sample.filename} must use image/png.`);
    }

    if (!Number.isInteger(sample.width) || sample.width <= 0) {
      errors.push(`Seed sample ${sample.filename} must define positive width.`);
    }

    if (!Number.isInteger(sample.height) || sample.height <= 0) {
      errors.push(`Seed sample ${sample.filename} must define positive height.`);
    }

    if (!Array.isArray(sample.annotations) || sample.annotations.length < 3) {
      errors.push(`Seed sample ${sample.filename} must define at least three annotations.`);
    }

    for (const annotation of sample.annotations ?? []) {
      if (!manifest.dataset.classes.includes(annotation.class_name)) {
        errors.push(`Annotation class ${annotation.class_name} is not in dataset classes.`);
      }

      for (const field of ['x', 'y', 'width', 'height']) {
        const value = annotation.bbox?.[field];
        if (typeof value !== 'number' || value < 0 || value > 1) {
          errors.push(`Annotation ${annotation.class_name} has invalid bbox ${field}.`);
        }
      }
    }

    if (!sample.yolo_label_path || !fs.existsSync(path.join(seedRoot, sample.yolo_label_path))) {
      errors.push(`YOLO label file is missing: ${sample.yolo_label_path}.`);
    } else {
      const yoloLines = fs
        .readFileSync(path.join(seedRoot, sample.yolo_label_path), 'utf8')
        .trim()
        .split('\n')
        .filter(Boolean);
      if (yoloLines.length !== sample.annotations.length) {
        errors.push(`YOLO labels for ${sample.filename} must match annotation count.`);
      }
    }
  }

  const script = fs.readFileSync('scripts/seed_demo_dataset.sh', 'utf8');
  if (script.includes('placeholder')) {
    errors.push('Seed script must not be a placeholder.');
  }

  return errors;
}
