use perception_app::DatasetStats;

use crate::dto::dataset_stats::DatasetStatsResponse;

pub fn dataset_stats_response(stats: DatasetStats) -> DatasetStatsResponse {
    DatasetStatsResponse {
        dataset_id: stats.dataset_id.to_string(),
        sample_count: stats.sample_count,
        annotation_count: stats.annotation_count,
        annotations_by_class: stats.annotations_by_class,
    }
}
