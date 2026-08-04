use crate::app::functions::{get::BoxedGet, run::BoxedRun, strategy::BoxedStrategy, view::View};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Settings<R, AE, AL, G, V, S, FD>
where
    R: BoxedRun,
    G: BoxedGet<R, AE, AL, FD>,
    V: View<R, AE, AL, FD>,
    S: BoxedStrategy,
{
    pub geter: G,
    pub viever: V,
    pub strategy: S,
    pub start_id: String,
    _m_phantom: PhantomData<(R, AE, AL, FD)>,
}

impl<R, AE, AL, G, V, S, FD> Settings<R, AE, AL, G, V, S, FD>
where
    R: BoxedRun,
    G: BoxedGet<R, AE, AL, FD>,
    V: View<R, AE, AL, FD>,
    S: BoxedStrategy,
{
    pub fn new(geter: G, viever: V, strategy: S, start_id: String) -> Self {
        Self {
            geter,
            viever,
            strategy,
            start_id,
            _m_phantom: PhantomData,
        }
    }
}
