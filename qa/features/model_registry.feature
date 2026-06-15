@p0 @api @database @storage
Feature: Model registry
  Successful training jobs must produce registered models with traceable artifacts.

  Scenario: Register model after successful training
    Given a successful training job exists and a model artifact exists in storage
    When the worker finalizes the training job
    Then a model should be created in the registry

  Scenario: Failed training job does not create model
    Given a training job has status "failed"
    When I retrieve models for this training job
    Then no model should exist for this training job

  Scenario: Retrieve model details
    Given a registered model exists
    When I call GET "/models/{model_id}"
    Then the response should contain model family, training job id, dataset version id, and metrics summary
