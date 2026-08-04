use std::{pin::Pin, time::Duration};

pub trait Strategy {
    fn start(&self, list_id: usize) -> impl Future<Output = usize> + Send;

    fn get_target_count(
        &self,
        list_id: usize,
        old_target_count: usize,
        result_count: usize,
        time: Duration,
    ) -> impl Future<Output = usize> + Send;
}

pub trait BoxedStrategy {
    fn start<'future>(
        &'future self,
        list_id: usize,
    ) -> Pin<Box<dyn Future<Output = usize> + Send + 'future>>;

    fn get_target_count<'future>(
        &'future self,
        list_id: usize,
        old_target_count: usize,
        result_count: usize,
        time: Duration,
    ) -> Pin<Box<dyn Future<Output = usize> + Send + 'future>>;
}

impl<T: Strategy> BoxedStrategy for T {
    fn start<'future>(
        &'future self,
        list_id: usize,
    ) -> Pin<Box<dyn Future<Output = usize> + Send + 'future>> {
        Box::pin(<Self as Strategy>::start(self, list_id))
    }

    fn get_target_count<'future>(
        &'future self,
        list_id: usize,
        old_target_count: usize,
        result_count: usize,
        time: Duration,
    ) -> Pin<Box<dyn Future<Output = usize> + Send + 'future>> {
        Box::pin(<Self as Strategy>::get_target_count(
            self,
            list_id,
            old_target_count,
            result_count,
            time,
        ))
    }
}

impl BoxedStrategy for Box<dyn BoxedStrategy> {
    fn start<'future>(
        &'future self,
        list_id: usize,
    ) -> Pin<Box<dyn Future<Output = usize> + Send + 'future>> {
        self.as_ref().start(list_id)
    }

    fn get_target_count<'future>(
        &'future self,
        list_id: usize,
        old_target_count: usize,
        result_count: usize,
        time: Duration,
    ) -> Pin<Box<dyn Future<Output = usize> + Send + 'future>> {
        self.as_ref()
            .get_target_count(list_id, old_target_count, result_count, time)
    }
}
