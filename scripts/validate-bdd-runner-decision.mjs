import { validateBddRunnerDecision } from './bdd-runner-policy.mjs';

const errors = validateBddRunnerDecision();

if (errors.length > 0) {
  console.error(errors.join('\n'));
  process.exit(1);
}

console.log('BDD runner decision validated.');
