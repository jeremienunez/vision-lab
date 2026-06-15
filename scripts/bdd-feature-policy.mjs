const requiredPatterns = [
  { label: 'tag', pattern: /^\s*@\w+/m },
  { label: 'Feature', pattern: /^\s*Feature:\s+.+/m },
  { label: 'Scenario', pattern: /^\s*Scenario:\s+.+/m },
  { label: 'Given', pattern: /^\s*Given\b.+/m },
  { label: 'When', pattern: /^\s*When\b.+/m },
  { label: 'Then', pattern: /^\s*Then\b.+/m },
];

export function validateFeatureDocument(content) {
  const feature = String(content ?? '');
  const errors = [];

  for (const requiredPattern of requiredPatterns) {
    if (!requiredPattern.pattern.test(feature)) {
      errors.push(`Missing required BDD feature element: ${requiredPattern.label}.`);
    }
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}
