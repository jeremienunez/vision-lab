@p0 @api
Feature: Standard API errors
  API errors must be predictable and safe.

  Scenario: Validation error response follows standard format
    Given the PerceptionLab API is running
    When I send an invalid request payload
    Then the response should contain error.code, error.message, and error.request_id

  Scenario: Internal errors do not expose stack traces
    Given the API encounters an internal error
    When I call the failing endpoint
    Then the response should not contain stack traces or file system paths
