@p2 @web @api
Feature: Operations dashboard
  The dashboard must expose the API-first MVP state without replacing backend source of truth.

  Scenario: Dashboard loads platform state from API endpoints
    Given the PerceptionLab API has datasets, training jobs, models, and metrics
    When I open the operations dashboard
    Then I should see dataset, active job, model, and latest metric summaries

  @smoke
  Scenario: Dashboard supports protected local APIs
    Given the PerceptionLab API is configured with an API key
    When I save the API key in the dashboard configuration panel
    Then protected dashboard API requests should include the x-api-key header

  Scenario: Dashboard shows API failures without fake data
    Given the PerceptionLab API is unavailable
    When I refresh the operations dashboard
    Then I should see an API error state
