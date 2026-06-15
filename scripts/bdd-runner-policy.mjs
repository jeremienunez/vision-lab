import fs from 'node:fs';

export function validateBddRunnerDecision() {
  const errors = [];
  const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
  const runnerDocPath = 'qa/bdd-runner.md';
  const runnerDoc = fs.existsSync(runnerDocPath)
    ? fs.readFileSync(runnerDocPath, 'utf8')
    : '';

  if (!packageJson.devDependencies?.['@cucumber/cucumber']) {
    errors.push('BDD runner dependency @cucumber/cucumber must be declared.');
  }

  if (!packageJson.scripts?.['bdd:dry-run']?.includes('cucumber-js')) {
    errors.push('package.json must expose bdd:dry-run using cucumber-js.');
  }

  if (!runnerDoc.includes('@cucumber/cucumber')) {
    errors.push('qa/bdd-runner.md must name @cucumber/cucumber as the final runner.');
  }

  for (const requiredText of [
    'qa/features/**/*.feature',
    'qa/steps/**/*.mjs',
    'qa/support/**/*.mjs',
    'qa/reports/cucumber-report.json',
  ]) {
    if (!runnerDoc.includes(requiredText)) {
      errors.push(`qa/bdd-runner.md must mention ${requiredText}.`);
    }
  }

  return errors;
}
