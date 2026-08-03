use iced::Element;

use crate::app::{
    CrabMenu,
    function::{Get, Run, Strategy, Viewer},
    message::Message,
};

impl<R, AE, AL, G, V, S> CrabMenu<R, AE, AL, G, V, S>
where
    R: Run,
    G: Get<R, AE, AL>,
    V: Viewer<R, AE, AL>,
    S: Strategy,
{
    pub fn view(&self) -> Element<'_, Message<R, AE, AL>> {
        self.settings.viever.view(&self.diagram)
    }
}
