use std::ops::Coroutine;
use std::pin::Pin;

pub struct IterCoro<T> {
    coro: Option<T>,
}

impl<T> Iterator for IterCoro<T>
where
    T: Coroutine + Unpin,
{
    type Item = T::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        let coro = self.coro.as_mut()?;

        match Pin::new(coro).resume(()) {
            std::ops::CoroutineState::Yielded(val) => Some(val),
            std::ops::CoroutineState::Complete(_) => {
                self.coro = None;
                None
            }
        }
    }
}

pub fn iter_coro<T>(coro: T) -> IterCoro<T>
where
    T: Coroutine,
{
    IterCoro { coro: Some(coro) }
}
