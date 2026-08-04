use crate::app::{
    CrabMenu,
    functions::{get::BoxedGet, run::BoxedRun, strategy::BoxedStrategy, view::View},
    message::Message,
};
use iced::Element;

impl<R, AE, AL, G, V, S, FD> CrabMenu<R, AE, AL, G, V, S, FD>
where
    R: BoxedRun,
    G: BoxedGet<R, AE, AL, FD>,
    V: View<R, AE, AL, FD>,
    S: BoxedStrategy,
{
    pub fn view(&self) -> Element<'_, Message<R, AE, AL, FD>> {
        self.settings.viever.view(&self.diagram, &self.find)
    }
}
