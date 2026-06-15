@p0 @api @database
Feature: Annotation management
  Users must be able to add object detection annotations to uploaded samples.

  Scenario: Add a valid bounding box annotation
    Given sample "cup.jpg" exists in a dataset with class "cup"
    When I add an annotation with bbox x 0.35 y 0.42 width 0.22 height 0.28
    Then the response status should be 201

  Scenario: Reject annotation with unknown class
    Given sample "cup.jpg" exists in a dataset with classes cup, book, phone
    When I add an annotation with class "bottle"
    Then the response status should be 422

  Scenario: Reject annotation outside image bounds
    Given sample "cup.jpg" exists
    When I add an annotation with bbox x 0.90 y 0.90 width 0.30 height 0.30
    Then the response should explain that the bounding box exceeds image bounds
