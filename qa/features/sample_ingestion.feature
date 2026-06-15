@p0 @api @storage @database
Feature: Sample ingestion
  Users must be able to upload valid images into datasets.

  Scenario: Upload a valid image sample
    Given dataset "desk-objects-v1" exists with classes cup, book, phone
    When I upload image "cup.jpg" to the dataset
    Then the response status should be 201

  Scenario: Reject unsupported file type
    Given dataset "desk-objects-v1" exists
    When I upload file "invalid.txt" to the dataset
    Then the response status should be 415

  Scenario: Detect duplicate image in the same dataset
    Given image "cup.jpg" was already uploaded to dataset "desk-objects-v1"
    When I upload image "cup.jpg" again to the same dataset
    Then the response status should be 409
