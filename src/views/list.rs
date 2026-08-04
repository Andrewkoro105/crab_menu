use crate::app::{
    element::Diagram,
    functions::{run::BoxedRun, view::View},
    message::Message,
};
use iced::{Element, widget::Id};
use iced::widget::{Column, button, column, row, text, text_input};
use std::collections::HashMap;

#[derive(Clone)]
pub struct List<AE, FD> {
    pub view: fn(&AE) -> String,
    pub to_find_data: fn(&String) -> FD,
    pub find_data_to_string: fn(&FD) -> String,
}

impl<R, AE, AL, FD> View<R, AE, AL, FD> for List<AE, FD>
where
    R: BoxedRun + Clone + 'static,
    AE: Clone + 'static,
    AL: Clone + 'static,
    FD: Clone + 'static,
{
    fn view<'element>(
        &'element self,
        data: &'element Diagram<R, AE, AL>,
        finds: &'element HashMap<usize, FD>,
    ) -> Element<'element, Message<R, AE, AL, FD>> {
        match data {
            Diagram::List(list) => column![
                if let Some(find_data) = finds.get(&list.id) {
                    text_input("", &(self.find_data_to_string)(find_data))
                        .on_input(|data| Message::Find {
                            id: list.id,
                            data: (self.to_find_data)(&data),
                        })
                        .id(list.id.to_string())
                        .into()
                } else {
                    Element::from(row![])
                },
                Column::from_iter(list.data.iter().map(|data| self.view(data, finds))).spacing(5)
            ]
            .spacing(5)
            .padding(10)
            .into(),
            Diagram::Element(element) => button(text!("{}", (self.view)(&element.additional_data)))
                .on_press(Message::Run(Box::new(element.run.clone())))
                .into(),
        }
    }
}
