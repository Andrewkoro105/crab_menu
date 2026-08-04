use crate::app::{element::Diagram, functions::run::BoxedRun};

pub trait Enter<R: BoxedRun, AE, AL> {
    fn enter(&self, data: &Diagram<R, AE, AL>) -> Option<R>;
}

impl<R: BoxedRun, AE, AL> Enter<R, AE, AL> for Box<dyn Enter<R, AE, AL>> {
    fn enter(&self, data: &Diagram<R, AE, AL>) -> Option<R> {
        self.as_ref().enter(data)
    }
}
