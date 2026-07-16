//! Coroutine contract shared by every People exchange.
//!
//! Defines the `PeopleCoroutine` trait, its `PeopleYield` and
//! `PeopleCoroutineState` companions, and the `people_try!` macro (the
//! coroutine equivalent of `?`).

use alloc::vec::Vec;

/// Progress of a coroutine after a resume: either an I/O request the
/// caller must fulfill, or the terminal value.
#[derive(Debug)]
pub enum PeopleCoroutineState<Y, R> {
    /// The coroutine needs I/O before it can go further.
    Yielded(Y),
    /// The coroutine finished with its terminal value.
    Complete(R),
}

/// A resumable, I/O-free People operation.
pub trait PeopleCoroutine {
    /// The I/O request type yielded while the exchange is in progress.
    type Yield;
    /// The terminal value type produced when the exchange completes.
    type Return;

    /// Advances the coroutine with the bytes read since the last yield
    /// (`None` when there is nothing to feed).
    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return>;
}

/// The I/O request a coroutine yields: read bytes from the stream, or
/// write the given bytes to it.
#[derive(Debug)]
pub enum PeopleYield {
    /// The coroutine wants bytes read from the stream.
    WantsRead,
    /// The coroutine wants the given bytes written to the stream.
    WantsWrite(Vec<u8>),
}

/// Resumes an inner coroutine, forwarding its yields and
/// short-circuiting its errors: the coroutine equivalent of `?`.
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
