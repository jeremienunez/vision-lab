@p0 @worker @ml @database
Feature: ML pipeline consistency
  The ML worker must preserve dataset, model, metric, and class consistency.

  Scenario: Model classes match dataset version classes
    Given dataset version "desk-objects-v1:v1" contains classes cup, book, and phone
    When a model is trained from this dataset version
    Then the registered model should contain classes cup, book, and phone

  Scenario: Training metrics are linked to model artifact
    Given a successful training job produced metrics and a model artifact
    When the model is registered
    Then the model metrics summary should match the training job final metrics

  Scenario: Inference classes must come from model metadata
    Given a model was trained with classes cup, book, and phone
    When I run inference with this model
    Then every detection class should be one of cup, book, or phone
