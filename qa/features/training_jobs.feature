@p0 @api @worker @ml @database
Feature: Training job orchestration
  Users must be able to launch asynchronous PyTorch training jobs from dataset versions.

  Scenario: Create a training job from a valid dataset version
    Given trainable dataset version "desk-objects-v1:v1" exists
    When I create a training job with model family "yolo" and base model "yolo11n"
    Then the job status should be "queued"

  Scenario: Worker completes a fake training job
    Given a training job is running in fake training mode
    When the worker completes the job successfully
    Then the job status should become "succeeded"

  Scenario: Failed training job stores an explicit error
    Given a training job is running
    When the worker marks the job as failed
    Then the job should contain an error message
