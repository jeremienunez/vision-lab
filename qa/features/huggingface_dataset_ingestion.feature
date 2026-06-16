@p2 @worker @ingestion
Feature: Hugging Face dataset ingestion
  External datasets must be materialized locally without leaking credentials.

  Scenario: Hugging Face dataset ingestion is materialized locally
    Given HF_TOKEN is configured and PERCEPTIONLAB_DATA_ROOT points to local storage
    When I ingest dataset "owner/desk-objects" with classes cup and book
    Then the worker writes images, YOLO labels, and a manifest under the target dataset folder

  Scenario: Missing Hugging Face token is rejected
    Given HF_TOKEN is not configured
    When I run the Hugging Face ingestion command
    Then the command fails before loading the external dataset

  Scenario: Hugging Face token is never leaked
    Given a Hugging Face loader fails while using a token
    When the worker reports the ingestion failure
    Then the error message and exception cause do not contain the token
