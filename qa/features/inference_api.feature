@p0 @api @inference @ml
Feature: Inference API
  Users must be able to run inference on an image using a registered model.

  Scenario: Run inference with a valid model and image
    Given a registered model exists with status "candidate"
    When I send image "cup.jpg" to POST "/models/{model_id}/infer"
    Then the response should contain model_id, latency_ms, and detections

  Scenario: Confidence threshold filters detections
    Given the inference confidence threshold is 0.90
    When I run inference on image "cup.jpg"
    Then every returned detection should have confidence greater than or equal to 0.90

  Scenario: Reject inference with invalid image file
    Given a registered model exists
    When I send file "invalid.txt" to POST "/models/{model_id}/infer"
    Then the response status should be 415
