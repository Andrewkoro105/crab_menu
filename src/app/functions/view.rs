use std::collections::HashMap;

use iced::Element;

use crate::app::{element::Diagram, functions::run::BoxedRun, message::Message};

pub trait View<R: BoxedRun, AE, AL, FD> {
    fn view<'element>(
        &'element self,
        data: &'element Diagram<R, AE, AL>,
        finds: &'element HashMap<usize, FD>,
    ) -> Element<'element, Message<R, AE, AL, FD>>;
}

impl<R: BoxedRun, AE, AL, FD> View<R, AE, AL, FD> for Box<dyn View<R, AE, AL, FD>> {
    fn view<'element>(
        &'element self,
        data: &'element Diagram<R, AE, AL>,
        finds: &'element HashMap<usize, FD>,
    ) -> Element<'element, Message<R, AE, AL, FD>> {
        self.as_ref().view(data, finds)
    }
}
