use crate::models::QueueItem;
use chrono::Utc;
use uuid::Uuid;

pub fn calculate_priority_score(item: &QueueItem) -> f64 {
    let age_minutes = (Utc::now() - item.created_at).num_minutes() as f64;
    let vote_boost = item.votes as f64 * 10.0;
    item.priority as f64 + vote_boost - (age_minutes / 60.0)
}

pub fn can_vote(user_id: &Uuid, item: &QueueItem) -> bool {
    user_id != &item.added_by
}
