@p2 @api @security
Feature: API key authentication
  The API must optionally protect non-health routes without breaking local development ergonomics.

  Background:
    Given the PerceptionLab API is running with API key "dev-secret"

  @smoke
  Scenario: Healthcheck remains public when API key auth is enabled
    When I call GET "/health" without an API key
    Then the response status should be 200

  @smoke
  Scenario: Protected route rejects missing API key
    When I call GET "/datasets" without an API key
    Then the response status should be 401
    And the response body should contain error code "missing_api_key"

  @smoke
  Scenario: Protected route rejects wrong API key
    When I call GET "/datasets" with API key "wrong-secret"
    Then the response status should be 403
    And the response body should contain error code "invalid_api_key"

  @smoke
  Scenario: Protected route accepts matching API key
    When I call GET "/datasets" with API key "dev-secret"
    Then the response should not be 401
    And the response should not be 403
