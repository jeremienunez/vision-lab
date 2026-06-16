@p1 @api @export @storage @ml
Feature: Model export
  Users must be able to export registered models to deployment formats.

  Scenario: Request ONNX export
    Given a registered model exists and its artifact is available
    When I request an export for the model with format "onnx"
    Then the response status should be 201

  Scenario: Successful ONNX export creates artifact reference
    Given a registered model exists and its artifact is available
    When I request an export for the model with format "onnx"
    Then the export status should be "succeeded" and include an ONNX artifact URI

  Scenario: CoreML export is available
    Given a registered model exists and its artifact is available
    When I request an export for the model with format "coreml"
    Then the export status should be "succeeded" and include a CoreML artifact URI

  Scenario: Reject unsupported export format
    Given a registered model exists
    When I request an export for the model with format "tflite"
    Then the response status should be 400
