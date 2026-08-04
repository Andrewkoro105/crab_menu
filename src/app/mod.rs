pub mod element;
pub mod functions;
pub mod message;
pub mod settings;
pub mod view;

use crate::app::{
    element::{Diagram, List},
    functions::{get::BoxedGet, run::BoxedRun, strategy::BoxedStrategy, view::View},
    message::Message,
    settings::Settings,
};
use iced::{
    Subscription, Task,
    application::BootFn,
    futures::{SinkExt, stream::BoxStream},
    stream,
};
use std::{collections::HashMap, hash::Hash, time::Instant};

pub struct CrabMenu<R, AE, AL, G, V, S, FD>
where
    R: BoxedRun,
    G: BoxedGet<R, AE, AL, FD>,
    V: View<R, AE, AL, FD>,
    S: BoxedStrategy,
{
    settings: Settings<R, AE, AL, G, V, S, FD>,
    diagram: Diagram<R, AE, AL>,
    find: HashMap<usize, FD>,
}

impl<R, AE, AL, G, V, S, FD> BootFn<CrabMenu<R, AE, AL, G, V, S, FD>, Message<R, AE, AL, FD>>
    for Settings<R, AE, AL, G, V, S, FD>
where
    R: BoxedRun + Clone + 'static,
    AE: Clone + 'static,
    AL: Default + Clone + 'static,
    G: BoxedGet<R, AE, AL, FD> + Clone,
    V: View<R, AE, AL, FD> + Clone,
    S: BoxedStrategy + Clone,
    FD: Clone + Default + 'static,
{
    fn boot(
        &self,
    ) -> (
        CrabMenu<R, AE, AL, G, V, S, FD>,
        iced::Task<Message<R, AE, AL, FD>>,
    ) {
        (
            CrabMenu {
                settings: self.clone(),
                diagram: Diagram::List(List {
                    id: 0,
                    additional_data: Default::default(),
                    data: vec![],
                }),
                find: HashMap::new(),
            },
            Task::done(Message::Find {
                id: 0,
                data: FD::default(),
            })
            .chain(Task::done(Message::SetFocus(self.start_id.clone()))),
        )
    }
}

impl<R, AE, AL, G, V, S, FD> CrabMenu<R, AE, AL, G, V, S, FD>
where
    R: BoxedRun + Send + Clone + 'static,
    AE: Send + Clone + 'static,
    AL: Send + Clone + 'static,
    G: BoxedGet<R, AE, AL, FD> + std::hash::Hash + Send + Clone + 'static,
    V: View<R, AE, AL, FD> + Clone + 'static,
    S: BoxedStrategy + std::hash::Hash + Send + Clone + 'static,
    FD: Hash + Send + Clone + 'static,
{
    pub fn subscription(&self) -> Subscription<Message<R, AE, AL, FD>> {
        Subscription::batch(self.find.iter().map(|(id, data)| {
            Subscription::run_with(
                (
                    id.clone(),
                    data.clone(),
                    self.settings.geter.clone(),
                    self.settings.strategy.clone(),
                ),
                CrabMenu::<R, AE, AL, G, V, S, FD>::find,
            )
        }))
    }

    fn find(data: &(usize, FD, G, S)) -> BoxStream<'static, Message<R, AE, AL, FD>> {
        let (id, find_data, getter, strategy) = data.clone();
        Box::pin(stream::channel(100, async move |mut output| {
            let mut target_count = strategy.start(id).await;
            let mut get_start = 0;
            loop {
                let start = Instant::now();
                let result = getter.get(&find_data, id, get_start, target_count).await;
                get_start += result.len();
                target_count = strategy
                    .get_target_count(id, target_count, result.len(), start.elapsed())
                    .await;
                output
                    .send(Message::FindResult { id, result })
                    .await
                    .unwrap();
            }
        }))
    }
}
