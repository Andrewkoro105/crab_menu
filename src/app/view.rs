use crate::app::{
    CrabMenu,
    functions::{enter::Enter, get::BoxedGet, run::BoxedRun, strategy::BoxedStrategy, view::View},
    message::Message,
};
use iced::Element;

impl<R, AE, AL, G, V, S, FD, E> CrabMenu<R, AE, AL, G, V, S, FD, E>
where
    R: BoxedRun,
    G: BoxedGet<R, AE, AL, FD>,
    V: View<R, AE, AL, FD>,
    S: BoxedStrategy,
    E: Enter<R, AE, AL>,
{
    pub fn view(&self) -> Element<'_, Message<R, AE, AL, FD>> {
        self.settings.viever.view(&self.diagram, &self.find)
    }
}
