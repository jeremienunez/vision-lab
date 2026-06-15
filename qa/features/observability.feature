@p1 @api @worker @nonfunctional
Feature: Observability
  The platform must expose enough information to debug requests, jobs, and failures.

  Scenario: API response contains request id
    Given the PerceptionLab API is running
    When I call GET "/health"
    Then the response should contain a request id header

  Scenario: Failed API request returns structured error
    Given the PerceptionLab API is running
    When I create a dataset with an invalid payload
    Then the response should contain an error code and human-readable message

  Scenario: Worker logs job lifecycle events
    Given a training job exists
    When the worker processes the training job
    Then the logs should contain the job id and status transitions
