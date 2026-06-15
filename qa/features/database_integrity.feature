@p0 @database
Feature: Database integrity
  The database must enforce referential integrity and protect core ML pipeline consistency.

  Scenario: Sample cannot exist without dataset
    Given the database schema is migrated
    When the system attempts to create a sample with an unknown dataset id
    Then the database should reject the operation

  Scenario: Training job cannot exist without dataset version
    Given the database schema is migrated
    When the system attempts to create a training job with an unknown dataset version id
    Then the database should reject the operation

  Scenario: Dataset version names are unique per dataset
    Given dataset "desk-objects-v1" has version "v1"
    When the system attempts to create another version "v1" for the same dataset
    Then the database should reject the duplicate version
