@p0 @storage @database
Feature: Artifacts storage consistency
  Files and database metadata must stay consistent across uploads, models, exports, and overlays.

  Scenario: Sample metadata is created only after successful file storage
    Given the storage service fails during upload
    When I upload image "cup.jpg"
    Then no sample metadata should be created in the database

  Scenario: Model artifact is stored after successful training
    Given a training job completes successfully
    When the worker saves the model artifact
    Then the model registry should reference the artifact URI

  Scenario: Artifact download refuses unknown artifact
    Given the PerceptionLab API is running
    When I request download for an unknown artifact id
    Then the response status should be 404
