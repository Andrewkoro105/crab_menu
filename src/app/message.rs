use crate::app::{
    CrabMenu,
    element::Diagram,
    function::{Get, Run, Strategy, Viewer},
};
use iced::Task;
use tracing::error;

pub enum Message<R: Run, AE, AL> {
    Run(Box<dyn Run>),
    Find {
        id: usize,
        data: String,
    },
    FindResult {
        id: usize,
        result: Vec<Diagram<R, AE, AL>>,
    },
}

unsafe impl<R: Run, AE, AL> Send for Message<R, AE, AL> {}

impl<R, AE, AL, G, V, S> CrabMenu<R, AE, AL, G, V, S>
where
    R: Run,
    G: Get<R, AE, AL>,
    V: Viewer<R, AE, AL>,
    S: Strategy,
{
    pub fn update(&mut self, message: Message<R, AE, AL>) -> Task<Message<R, AE, AL>> {
        match message {
            Message::Run(run) => {
                run.run();
                Task::none()
            }
            Message::Find { id, data } => {
                self.find.insert(id, data);
                Task::none()
            }
            Message::FindResult { id, result } => {
                if let Some(list) = self.diagram.find_list_mut(id) {
                    list.data.extend(result);
                } else {
                    error!("There is no entry with ID {}", id);
                }
                Task::none()
            }
        }
    }
}
