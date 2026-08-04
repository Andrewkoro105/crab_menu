use std::time::Duration;

use crate::app::functions::strategy::Strategy;

#[derive(Clone, Hash)]
pub struct AdaptiveQueryLimiter {
    pub target_time: Duration,
    pub start_target_count: usize,
    pub delay_at_zero: Duration,
}

impl Strategy for AdaptiveQueryLimiter {
    async fn start(&self, _list_id: usize) -> usize {
        self.start_target_count
    }

    async fn get_target_count(
        &self,
        _list_id: usize,
        old_target_count: usize,
        result_count: usize,
        time: Duration,
    ) -> usize {
        if result_count == 0 {
            tokio::time::sleep(self.delay_at_zero).await;
            self.start_target_count
        } else {
            if time < self.target_time {
                old_target_count * 2
            } else {
                old_target_count / 2 + old_target_count / 4
            }
        }
    }
}
