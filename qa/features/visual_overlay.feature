@p1 @api @inference
Feature: Visual overlay generation
  Users must be able to generate annotated images from inference results.

  Scenario: Generate overlay from inference run
    Given an inference run exists with detections
    When I request an overlay for the inference run
    Then the response should contain an overlay artifact URI

  Scenario: Overlay contains class name and confidence
    Given the inference run contains a detection for class "cup" with confidence 0.89
    When I generate an overlay
    Then the overlay label should contain "cup" and "89%"

  Scenario: Reject overlay generation for unknown inference run
    Given the PerceptionLab API is running
    When I request an overlay for an unknown inference run id
    Then the response status should be 404
