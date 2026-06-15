use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DomainError;

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }

            pub fn parse(value: impl AsRef<str>) -> Result<Self, DomainError> {
                Uuid::from_str(value.as_ref())
                    .map(Self)
                    .map_err(|_| DomainError::InvalidId)
            }

            pub fn into_uuid(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

define_id!(DatasetId);
define_id!(SampleId);
define_id!(AnnotationId);
define_id!(DatasetVersionId);
define_id!(TrainingJobId);
define_id!(TrainingMetricId);
define_id!(ModelId);
define_id!(ModelExportId);
define_id!(ArtifactId);
