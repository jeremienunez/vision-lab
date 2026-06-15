@p0 @api @database
Feature: Dataset management
  Users must be able to create and inspect datasets for computer vision training.

  Scenario: Create a valid object detection dataset
    Given the PerceptionLab API is running
    When I create dataset "desk-objects-v1" with task type "object_detection" and classes cup, book, phone, keyboard, mouse
    Then the response status should be 201

  Scenario: Reject duplicate dataset name
    Given a dataset named "desk-objects-v1" already exists
    When I create another dataset named "desk-objects-v1"
    Then the response status should be 409

  Scenario: Retrieve dataset statistics
    Given a dataset named "desk-objects-v1" contains 2 samples and 3 annotations
    When I call GET "/datasets/{dataset_id}/stats"
    Then the response should contain sample_count 2 and annotation_count 3
