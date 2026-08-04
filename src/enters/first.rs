use crate::app::{
    element::Diagram,
    functions::{enter::Enter, run::BoxedRun},
};

#[derive(Clone)]
pub struct First;

impl<R: BoxedRun + Clone, AE, AL> Enter<R, AE, AL> for First {
    fn enter(&self, data: &Diagram<R, AE, AL>) -> Option<R> {
        match data {
            Diagram::List(list) => list.data.first().map(|data| self.enter(data)).flatten(),
            Diagram::Element(element) => Some(element.run.clone()),
        }
    }
}
