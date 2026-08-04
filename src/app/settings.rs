use crate::app::functions::{enter::Enter, get::BoxedGet, run::BoxedRun, strategy::BoxedStrategy, view::View};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Settings<R, AE, AL, G, V, S, FD, E>
where
    R: BoxedRun,
    G: BoxedGet<R, AE, AL, FD>,
    V: View<R, AE, AL, FD>,
    S: BoxedStrategy,
    E: Enter<R, AE, AL>,
{
    pub geter: G,
    pub viever: V,
    pub strategy: S,
    pub enter: E,
    pub start_id: String,
    _m_phantom: PhantomData<(R, AE, AL, FD)>,
}

impl<R, AE, AL, G, V, S, FD, E> Settings<R, AE, AL, G, V, S, FD, E>
where
    R: BoxedRun,
    G: BoxedGet<R, AE, AL, FD>,
    V: View<R, AE, AL, FD>,
    S: BoxedStrategy,
    E: Enter<R, AE, AL>,
{
    pub fn new(geter: G, viever: V, strategy: S, enter: E, start_id: String) -> Self {
        Self {
            geter,
            viever,
            strategy,
            enter,
            start_id,
            _m_phantom: PhantomData,
        }
    }
}
