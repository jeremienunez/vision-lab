@p2 @api @security
Feature: API key authentication
  Protected API routes must require an API key only when local configuration enables it.

  Scenario: Healthcheck remains public when API key auth is enabled
    Given the PerceptionLab API is configured with an API key
    When I call the healthcheck endpoint without an API key
    Then the response status should be 200

  Scenario: Protected route rejects missing API key
    Given the PerceptionLab API is configured with an API key
    When I call a protected endpoint without the x-api-key header
    Then the response status should be 401

  Scenario: Protected route rejects wrong API key
    Given the PerceptionLab API is configured with an API key
    When I call a protected endpoint with the wrong x-api-key header
    Then the response status should be 403

  Scenario: Protected route accepts correct API key
    Given the PerceptionLab API is configured with an API key
    When I call a protected endpoint with the correct x-api-key header
    Then the response status should be 200
