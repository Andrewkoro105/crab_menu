use std::pin::Pin;

use crate::app::{element::Diagram, functions::run::BoxedRun};

pub trait Get<R: BoxedRun, AE, AL, FD> {
    fn get(
        &self,
        find_data: &FD,
        id: usize,
        start: usize,
        target_count: usize,
    ) -> impl Future<Output = Vec<Diagram<R, AE, AL>>> + Send;
}

pub trait BoxedGet<R: BoxedRun, AE, AL, FD> {
    fn get<'future>(
        &'future self,
        find_data: &'future FD,
        id: usize,
        start: usize,
        target_count: usize,
    ) -> Pin<Box<dyn Future<Output = Vec<Diagram<R, AE, AL>>> + Send + 'future>>
    where
        R: 'future,
        AE: 'future,
        AL: 'future;
}

#[derive(Clone, Hash)]
pub struct BoxedGetAdapter<T>(pub T);

impl<R, AE, AL, T: Get<R, AE, AL, FD>, FD> BoxedGet<R, AE, AL, FD> for BoxedGetAdapter<T>
where
    R: BoxedRun,
{
    fn get<'future>(
        &'future self,
        find_data: &'future FD,
        id: usize,
        start: usize,
        target_count: usize,
    ) -> Pin<Box<dyn Future<Output = Vec<Diagram<R, AE, AL>>> + Send + 'future>>
    where
        R: 'future,
        AE: 'future,
        AL: 'future,
    {
        Box::pin(self.0.get(find_data, id, start, target_count))
    }
}

impl<R, AE, AL, FD> BoxedGet<R, AE, AL, FD> for Box<dyn BoxedGet<R, AE, AL, FD>>
where
    R: BoxedRun,
{
    fn get<'future>(
        &'future self,
        find_data: &'future FD,
        id: usize,
        start: usize,
        target_count: usize,
    ) -> Pin<Box<dyn Future<Output = Vec<Diagram<R, AE, AL>>> + Send + 'future>>
    where
        R: 'future,
        AE: 'future,
        AL: 'future,
    {
        self.as_ref().get(find_data, id, start, target_count)
    }
}
