@p1 @nonfunctional @performance
Feature: Performance smoke tests
  The platform must meet minimal performance expectations for the MVP demo.

  Scenario: API creates dataset quickly
    Given the PerceptionLab stack is running locally
    When I create a valid dataset
    Then the response time should be below 500 milliseconds

  Scenario: Training job creation is asynchronous
    Given a trainable dataset version exists
    When I create a training job
    Then the response time should be below 500 milliseconds

  Scenario: Inference returns within acceptable local MVP latency
    Given a registered tiny model exists
    When I run inference on image "cup.jpg"
    Then the response time should be below 1000 milliseconds
