use crate::app::{element::Diagram, message::Message};
use iced::Element;
use std::{pin::Pin, time::Duration};

pub trait Get<R: Run, AE, AL> {
    fn get(
        &self,
        find_data: &String,
        id: usize,
        start: usize,
        target_count: usize,
    ) -> impl Future<Output = Vec<Diagram<R, AE, AL>>> + Send;
}

pub trait BoxedGet<'future, R: Run, AE, AL> {
    fn get(
        &'future self,
        find_data: &'future String,
        id: usize,
        start: usize,
        target_count: usize,
    ) -> Pin<Box<dyn Future<Output = Vec<Diagram<R, AE, AL>>> + Send + 'future>>;
}

pub trait Viewer<R: Run, AE, AL> {
    fn view(&self, data: &Diagram<R, AE, AL>) -> Element<'_, Message<R, AE, AL>>;
}

pub trait Run {
    fn run(&self);
}

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

pub struct BoxedGetAdapter<T>(pub T);

impl<'future, R, AE, AL, T: Get<R, AE, AL>> BoxedGet<'future, R, AE, AL> for BoxedGetAdapter<T>
where
    R: Run + 'future,
    AE: 'future,
    AL: 'future,
{
    fn get(
        &'future self,
        find_data: &'future String,
        id: usize,
        start: usize,
        target_count: usize,
    ) -> Pin<Box<dyn Future<Output = Vec<Diagram<R, AE, AL>>> + Send + 'future>> {
        Box::pin(self.0.get(find_data, id, start, target_count))
    }
}

impl<'future, R, AE, AL> BoxedGet<'future, R, AE, AL> for Box<dyn BoxedGet<'future, R, AE, AL>>
where
    R: Run + 'future,
    AE: 'future,
    AL: 'future,
{
    fn get(
        &'future self,
        find_data: &'future String,
        id: usize,
        start: usize,
        target_count: usize,
    ) -> Pin<Box<dyn Future<Output = Vec<Diagram<R, AE, AL>>> + Send + 'future>> {
        self.as_ref().get(find_data, id, start, target_count)
    }
}

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
            0
        } else {
            if time < self.target_time {
                old_target_count * 2
            } else {
                old_target_count / 2 + old_target_count / 4
            }
        }
    }
}
