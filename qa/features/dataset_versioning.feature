@p0 @api @database
Feature: Dataset versioning
  Users must be able to freeze datasets into immutable versions used by training jobs.

  Scenario: Create a dataset version
    Given dataset "desk-objects-v1" contains annotated samples
    When I create dataset version "v1"
    Then the response status should be 201

  Scenario: Dataset version is immutable after dataset changes
    Given dataset version "v1" exists
    When I upload a new image and annotation to the dataset
    Then dataset version "v1" should keep its original sample and annotation counts

  Scenario: Training must use a dataset version
    Given a mutable dataset exists
    When I create a training job using a dataset id instead of a dataset version id
    Then the response status should be 422
