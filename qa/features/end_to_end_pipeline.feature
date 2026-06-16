@p0 @e2e @api @storage @database @worker @ml
Feature: End-to-end computer vision pipeline
  PerceptionLab must transform raw visual data into a trained and usable model.

  Scenario: Build a vision model from ingested data
    Given the PerceptionLab stack is running
    When I create dataset "desk-objects-v1", upload images, add annotations, create version "v1", and launch a training job
    Then the training job status should be "queued"

  Scenario: Complete fake training and run inference
    Given the worker processes the training job in fake training mode
    When I run inference with the registered model on image "cup.jpg"
    Then the inference response should contain detections

  Scenario: Generate a visual overlay
    Given an inference run exists with detections
    When I generate a visual overlay from the inference result
    Then an overlay image should be created in storage

  @p2 @demo
  Scenario: Fire the local object-recognition smoke
    Given the local transient API can start
    When I run the product fire smoke
    Then the summary should include detected classes and an overlay artifact URI
