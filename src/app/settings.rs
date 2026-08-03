use crate::app::function::{Get, Run, Strategy, Viewer};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Settings<R, AE, AL, G, V, S>
where
    R: Run,
    G: Get<R, AE, AL>,
    V: Viewer<R, AE, AL>,
    S: Strategy,
{
    pub geter: G,
    pub viever: V,
    pub strategy: S,
    _m_phantom: PhantomData<(R, AE, AL)>,
}
