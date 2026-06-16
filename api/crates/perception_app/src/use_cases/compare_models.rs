use std::{cmp::Ordering, collections::BTreeSet};

use perception_domain::ModelId;

use crate::{ModelComparison, ModelComparisonEntry, ModelRepository, UseCaseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompareModelsCommand {
    pub model_ids: Vec<ModelId>,
    pub metric_name: String,
}

pub struct CompareModelsUseCase<'repository> {
    model_repository: &'repository dyn ModelRepository,
}

impl<'repository> CompareModelsUseCase<'repository> {
    pub fn new(model_repository: &'repository dyn ModelRepository) -> Self {
        Self { model_repository }
    }

    pub async fn execute(
        &self,
        command: CompareModelsCommand,
    ) -> Result<ModelComparison, UseCaseError> {
        let metric_name = command.metric_name.trim().to_owned();

        if metric_name.is_empty() {
            return Err(UseCaseError::Validation(
                "model comparison metric is required",
            ));
        }

        let unique_ids = command
            .model_ids
            .iter()
            .map(ToString::to_string)
            .collect::<BTreeSet<_>>();

        if unique_ids.len() < 2 || unique_ids.len() != command.model_ids.len() {
            return Err(UseCaseError::Validation(
                "model comparison requires at least two models",
            ));
        }

        let direction = comparison_direction(&metric_name);
        let mut entries = Vec::with_capacity(command.model_ids.len());

        for model_id in command.model_ids {
            let model = self
                .model_repository
                .get(model_id)
                .await?
                .ok_or(UseCaseError::NotFound("model not found"))?;
            let metric_value = model
                .metrics_summary
                .get(&metric_name)
                .ok_or(UseCaseError::Validation(
                    "models must have comparable metric",
                ))?
                .parse::<f64>()
                .map_err(|_| UseCaseError::Validation("models must have comparable metric"))?;

            if !metric_value.is_finite() {
                return Err(UseCaseError::Validation(
                    "models must have comparable metric",
                ));
            }

            entries.push(ModelComparisonEntry {
                rank: 0,
                model_id: model.id,
                name: model.name,
                version: model.version,
                metric_value,
                metrics_summary: model.metrics_summary,
            });
        }

        entries.sort_by(|left, right| compare_entries(left, right, direction));

        for (index, entry) in entries.iter_mut().enumerate() {
            entry.rank = (index + 1) as u32;
        }

        let best_model_id = entries
            .first()
            .ok_or(UseCaseError::Validation(
                "model comparison requires at least two models",
            ))?
            .model_id;

        Ok(ModelComparison {
            metric_name,
            direction: direction.as_str(),
            best_model_id,
            models: entries,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComparisonDirection {
    HigherIsBetter,
    LowerIsBetter,
}

impl ComparisonDirection {
    fn as_str(self) -> &'static str {
        match self {
            Self::HigherIsBetter => "higher_is_better",
            Self::LowerIsBetter => "lower_is_better",
        }
    }
}

fn comparison_direction(metric_name: &str) -> ComparisonDirection {
    if metric_name.to_ascii_lowercase().contains("loss") {
        ComparisonDirection::LowerIsBetter
    } else {
        ComparisonDirection::HigherIsBetter
    }
}

fn compare_entries(
    left: &ModelComparisonEntry,
    right: &ModelComparisonEntry,
    direction: ComparisonDirection,
) -> Ordering {
    let metric_order = match direction {
        ComparisonDirection::HigherIsBetter => right
            .metric_value
            .partial_cmp(&left.metric_value)
            .expect("metric values are finite"),
        ComparisonDirection::LowerIsBetter => left
            .metric_value
            .partial_cmp(&right.metric_value)
            .expect("metric values are finite"),
    };

    metric_order.then_with(|| left.model_id.to_string().cmp(&right.model_id.to_string()))
}
