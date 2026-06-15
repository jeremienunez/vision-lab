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
