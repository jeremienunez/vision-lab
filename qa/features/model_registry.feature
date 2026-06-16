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

  Scenario: Models can be compared
    Given two registered models have comparable validation metrics
    When I compare the models
    Then the response ranks the model with the best validation metric first

  Scenario: Model promotion is exclusive
    Given two models exist for the same dataset version and model family
    When I promote one model
    Then that model status becomes "promoted" and no competing model remains promoted
