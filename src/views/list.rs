use crate::app::{
    element::Diagram,
    functions::{run::BoxedRun, view::View},
    message::Message,
};
use iced::Element;
use iced::widget::{button, column, row, text, text_input};
use iced_helper::widgets::virtualized_list::virtualized_list;
use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
pub struct List<AE, FD> {
    pub view_elem: fn(&AE) -> String,
    pub to_find_data: fn(&String) -> FD,
    pub find_data_to_string: fn(&FD) -> String,
}

impl<R, AE, AL, FD> View<R, AE, AL, FD> for List<AE, FD>
where
    R: BoxedRun + Hash + Clone + 'static,
    AE: Hash + Clone + 'static,
    AL: Hash + Clone + 'static,
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
                if !list.data.is_empty() {
                    virtualized_list(&list.data)
                        .context((self, finds))
                        .get_elem(|data, _, (this, finds)| this.view(data, finds))
                        .spacing(10)
                        .into()
                } else {
                    Element::from(row![])
                }
            ]
            .spacing(5)
            .padding(10)
            .into(),
            Diagram::Element(element) => {
                button(text!("{}", (self.view_elem)(&element.additional_data)))
                    .on_press(Message::Run(Box::new(element.run.clone())))
                    .into()
            }
        }
    }
}
