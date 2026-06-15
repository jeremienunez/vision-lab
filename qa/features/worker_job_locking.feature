@p0 @worker
Feature: Worker job locking
  Training jobs must not be processed twice.

  Scenario: Two workers cannot process the same job
    Given a training job exists with status "queued"
    When two workers poll for jobs at the same time
    Then only one worker should process the job
