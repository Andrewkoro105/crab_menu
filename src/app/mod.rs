use std::{collections::HashMap, time::Instant};

use crate::app::{
    element::{Diagram, List},
    function::{Get, Run, Strategy, Viewer},
    message::Message,
    settings::Settings,
};
use iced::{
    Subscription, Task,
    application::BootFn,
    futures::{SinkExt, stream::BoxStream},
    stream,
};
pub mod element;
pub mod function;
pub mod message;
pub mod settings;
pub mod view;

pub struct CrabMenu<R, AE, AL, G, V, S>
where
    R: Run,
    G: Get<R, AE, AL>,
    V: Viewer<R, AE, AL>,
    S: Strategy,
{
    settings: Settings<R, AE, AL, G, V, S>,
    diagram: Diagram<R, AE, AL>,
    find: HashMap<usize, String>,
}

impl<R, AE, AL, G, V, S> BootFn<CrabMenu<R, AE, AL, G, V, S>, Message<R, AE, AL>>
    for Settings<R, AE, AL, G, V, S>
where
    R: Run + Clone + 'static,
    AE: Clone + 'static,
    AL: Default + Clone + 'static,
    G: Get<R, AE, AL> + Clone,
    V: Viewer<R, AE, AL> + Clone,
    S: Strategy + Clone,
{
    fn boot(&self) -> (CrabMenu<R, AE, AL, G, V, S>, iced::Task<Message<R, AE, AL>>) {
        (
            CrabMenu {
                settings: self.clone(),
                diagram: Diagram::List(List {
                    id: 0,
                    additional_data: Default::default(),
                    data: vec![]
                }),
                find: HashMap::new(),
            },
            Task::done(Message::Find { id: 0, data: "".into() }),
        )
    }
}

impl<R, AE, AL, G, V, S> CrabMenu<R, AE, AL, G, V, S>
where
    R: Run + Send + Clone + 'static,
    AE: Send + Clone + 'static,
    AL: Send + Clone + 'static,
    G: Get<R, AE, AL> + std::hash::Hash + Send + Clone + 'static,
    V: Viewer<R, AE, AL> + Clone + 'static,
    S: Strategy + std::hash::Hash + Send + Clone + 'static,
{
    pub fn subscription(&self) -> Subscription<Message<R, AE, AL>> {
        Subscription::batch(self.find.iter().map(|(id, data)| {
            Subscription::run_with(
                (
                    id.clone(),
                    data.clone(),
                    self.settings.geter.clone(),
                    self.settings.strategy.clone(),
                ),
                CrabMenu::<R, AE, AL, G, V, S>::find,
            )
        }))
    }

    fn find(data: &(usize, String, G, S)) -> BoxStream<'static, Message<R, AE, AL>> {
        let (id, find_data, getter, strategy) = data.clone();
        Box::pin(stream::channel(100, async move |mut output| {
            let mut target_count = strategy.start(id).await;
            loop {
                let start = Instant::now();
                let result = getter.get(&find_data, id, 0, target_count).await;
                target_count = strategy.get_target_count(id, target_count, result.len(), start.elapsed()).await;
                output
                    .send(Message::FindResult { id, result })
                    .await
                    .unwrap();
            }
        }))
    }
}
