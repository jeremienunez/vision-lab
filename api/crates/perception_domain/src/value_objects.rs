use serde::{Deserialize, Serialize};

use crate::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NormalizedBbox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl NormalizedBbox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Result<Self, DomainError> {
        let bbox = Self {
            x,
            y,
            width,
            height,
        };

        if bbox.is_valid() {
            Ok(bbox)
        } else {
            Err(DomainError::InvalidNormalizedBbox)
        }
    }

    fn is_valid(&self) -> bool {
        let values = [self.x, self.y, self.width, self.height];

        values.iter().all(|value| (0.0..=1.0).contains(value))
            && self.width > 0.0
            && self.height > 0.0
            && self.x + self.width <= 1.0
            && self.y + self.height <= 1.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

impl ImageDimensions {
    pub fn new(width: u32, height: u32) -> Result<Self, DomainError> {
        if width == 0 || height == 0 {
            Err(DomainError::InvalidImageDimensions)
        } else {
            Ok(Self { width, height })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TrainingHyperparameters {
    pub epochs: u16,
    pub batch_size: u16,
    pub image_size: u16,
    pub learning_rate: f32,
}

impl TrainingHyperparameters {
    pub fn new(
        epochs: u16,
        batch_size: u16,
        image_size: u16,
        learning_rate: f32,
    ) -> Result<Self, DomainError> {
        if epochs == 0 || batch_size == 0 || image_size == 0 || learning_rate <= 0.0 {
            return Err(DomainError::InvalidTrainingHyperparameters);
        }

        Ok(Self {
            epochs,
            batch_size,
            image_size,
            learning_rate,
        })
    }
}
