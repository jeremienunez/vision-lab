@p0 @api @security
Feature: Basic API security
  The API must reject unsafe inputs and keep internal details private.

  Scenario: Uploaded filename is sanitized
    Given dataset "desk-objects-v1" exists
    When I upload a file named "../../secret.jpg"
    Then the upload should not write outside the storage directory

  Scenario: Error response does not leak environment variables
    Given the API encounters an internal error
    When I call the failing endpoint
    Then the response should not contain environment variable values

  Scenario: API rejects unsupported content type
    Given the PerceptionLab API is running
    When I send a JSON payload to an endpoint expecting multipart image upload
    Then the response status should be 415
