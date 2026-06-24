import { setWorldConstructor } from '@cucumber/cucumber';

class PerceptionLabWorld {
  constructor() {
    this.apiKey = '';
    this.response = null;
    this.dashboardHeaders = {};
    this.fireSummary = null;
  }
}

setWorldConstructor(PerceptionLabWorld);
