@p1 @api @export @storage @ml
Feature: Model export
  Users must be able to export registered models to deployment formats.

  Scenario: Request ONNX export
    Given a registered model exists and its artifact is available
    When I request an export for the model with format "onnx"
    Then the response status should be 202

  Scenario: Successful ONNX export creates artifact
    Given an ONNX export is queued
    When the worker completes the export successfully
    Then the export status should become "succeeded"

  Scenario: Reject unsupported export format
    Given a registered model exists
    When I request an export for the model with format "tflite"
    Then the response status should be 422
