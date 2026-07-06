//! Coroutine driver: the `PeopleCoroutine` trait, its `PeopleYield` /
//! `PeopleCoroutineState`, and the `people_try!` macro (the coroutine
//! equivalent of `?`).

use alloc::vec::Vec;

#[derive(Debug)]
pub enum PeopleCoroutineState<Y, R> {
    Yielded(Y),
    Complete(R),
}

pub trait PeopleCoroutine {
    type Yield;
    type Return;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return>;
}

#[derive(Debug)]
pub enum PeopleYield {
    WantsRead,
    WantsWrite(Vec<u8>),
}

#[macro_export]
macro_rules! people_try {
    ($coroutine:expr, $arg:expr $(,)?) => {
        match $crate::coroutine::PeopleCoroutine::resume($coroutine, $arg) {
            $crate::coroutine::PeopleCoroutineState::Yielded(y) => {
                return $crate::coroutine::PeopleCoroutineState::Yielded(y.into());
            }
            $crate::coroutine::PeopleCoroutineState::Complete(Err(err)) => {
                log::trace!("error during coroutine execution: {err}");
                return $crate::coroutine::PeopleCoroutineState::Complete(Err(err.into()));
            }
            $crate::coroutine::PeopleCoroutineState::Complete(Ok(value)) => value,
        }
    };
}
