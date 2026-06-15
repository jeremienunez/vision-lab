/** @type {import('dependency-cruiser').IConfiguration} */
module.exports = {
  forbidden: [
    {
      name: 'no-circular',
      severity: 'error',
      from: {},
      to: {
        circular: true
      }
    },
    {
      name: 'domain-is-pure',
      severity: 'error',
      comment: 'Domain code must not depend on application, infrastructure, or presentation layers.',
      from: {
        path: '^src/domain'
      },
      to: {
        path: '^src/(application|infrastructure|presentation)'
      }
    },
    {
      name: 'application-ignores-adapters',
      severity: 'error',
      comment: 'Application code can use domain contracts, but not concrete infrastructure or UI adapters.',
      from: {
        path: '^src/application'
      },
      to: {
        path: '^src/(infrastructure|presentation)'
      }
    },
    {
      name: 'presentation-does-not-reach-infrastructure',
      severity: 'error',
      comment: 'Presentation must call application services, not infrastructure implementations directly.',
      from: {
        path: '^src/presentation'
      },
      to: {
        path: '^src/infrastructure'
      }
    },
    {
      name: 'no-orphans-in-src',
      severity: 'warn',
      from: {
        orphan: true,
        path: '^src/'
      },
      to: {}
    }
  ],
  options: {
    doNotFollow: {
      path: 'node_modules'
    },
    exclude: {
      path: 'node_modules|coverage|dist|build'
    },
    tsPreCompilationDeps: false,
    reporterOptions: {
      text: {
        highlightFocused: true
      }
    }
  }
};
