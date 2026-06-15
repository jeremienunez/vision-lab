@p0 @api
Feature: Platform healthcheck
  The platform must expose health endpoints so users and automation can verify service readiness.

  Scenario: API healthcheck returns healthy status
    Given the PerceptionLab API is running
    When I call GET "/health"
    Then the response status should be 200

  Scenario: API healthcheck exposes dependency status
    Given the PerceptionLab API is running
    When I call GET "/health"
    Then the response body should contain database, storage, and queue status
