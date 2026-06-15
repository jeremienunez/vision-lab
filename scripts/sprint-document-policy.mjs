const requiredSections = [
  { label: 'Goal', pattern: /^## Goal\s*$/im },
  { label: 'Priority', pattern: /^## Priority\s*$/im },
  { label: 'Dependencies', pattern: /^## Dependencies\s*$/im },
  { label: 'Scope', pattern: /^## Scope\s*$/im },
  { label: 'BDD Validation Criteria', pattern: /^## BDD Validation Criteria\s*$/im },
  { label: 'Definition of Done', pattern: /^## Definition of Done\s*$/im },
];

const requiredBddTerms = [
  { label: 'Given', pattern: /^\s*Given\b/im },
  { label: 'When', pattern: /^\s*When\b/im },
  { label: 'Then', pattern: /^\s*Then\b/im },
];

export function validateSprintDocument(content) {
  const markdown = String(content ?? '');
  const errors = [];

  for (const section of requiredSections) {
    if (!section.pattern.test(markdown)) {
      errors.push(`Missing required section: ${section.label}.`);
    }
  }

  for (const term of requiredBddTerms) {
    if (!term.pattern.test(markdown)) {
      errors.push(`Missing BDD term: ${term.label}.`);
    }
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}
