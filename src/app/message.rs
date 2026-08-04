use crate::app::{
    CrabMenu,
    element::Diagram,
    functions::{enter::Enter, get::BoxedGet, run::BoxedRun, strategy::BoxedStrategy, view::View},
};
use iced::{Task, exit, widget::operation::focus};
use tracing::error;

#[derive(Clone)]
pub enum Message<R: BoxedRun, AE, AL, FD> {
    Run(Box<dyn BoxedRun>),
    Find {
        id: usize,
        data: FD,
    },
    FindResult {
        id: usize,
        result: Vec<Diagram<R, AE, AL>>,
    },
    SetFocus(String),
    Enter,
}

unsafe impl<R: BoxedRun, AE, AL, FD> Send for Message<R, AE, AL, FD> {}

impl<R, AE, AL, G, V, S, FD, E> CrabMenu<R, AE, AL, G, V, S, FD, E>
where
    R: BoxedRun + 'static,
    AE: 'static,
    AL: 'static,
    G: BoxedGet<R, AE, AL, FD>,
    V: View<R, AE, AL, FD>,
    S: BoxedStrategy,
    FD: 'static,
    E: Enter<R, AE, AL>,
{
    pub fn update(&mut self, message: Message<R, AE, AL, FD>) -> Task<Message<R, AE, AL, FD>> {
        match message {
            Message::Run(run) => {
                let close_it = run.close_it();
                Task::future(async move { run.run().await }).discard().chain(if close_it {
                    exit()
                } else {
                    Task::none()
                })
            },
            Message::Find { id, data } => {
                if let Some(list) = self.diagram.find_list_mut(id) {
                    list.data.clear();
                } else {
                    error!("There is no entry with ID {}", id);
                }
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
            Message::SetFocus(id) => focus(id),
            Message::Enter => if let Some(run) = self.settings.enter.enter(&self.diagram) {
                Task::done(Message::Run(Box::new(run)))
            } else {
                Task::none()
            }
        }
    }
}
