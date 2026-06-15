@p0 @api @worker @database @ml
Feature: Metrics tracking
  Training jobs must expose metrics so users can evaluate model quality.

  Scenario: Worker writes metrics for each epoch
    Given a training job is running
    When the worker completes epoch 1 with training metrics
    Then the metrics should be persisted for epoch 1

  Scenario: Retrieve metrics for a training job
    Given a training job has metrics for 2 epochs
    When I call GET "/training-jobs/{job_id}/metrics"
    Then the response should contain 2 metric records ordered by epoch

  Scenario: Training job summary exposes best epoch
    Given a training job has metrics for multiple epochs
    When I retrieve the training job details
    Then the response should contain the best mAP50 and best epoch
