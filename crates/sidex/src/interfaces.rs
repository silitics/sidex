//! Abstractions for [_interfaces_](https://oss.silitics.com/sidex/docs/guide/interfaces).

/// A method of an interface which can be invoked.
pub trait Method {
    /// The return type of the method.
    type Output;
}

impl<'m, M: Method> Method for &'m M {
    type Output = M::Output;
}

impl<'m, M: Method> Method for &'m mut M {
    type Output = M::Output;
}

/// Invocation _output modes_.
pub mod output {
    use core::{future::Future, marker::PhantomData, pin::Pin};

    use super::OutputMode;

    pub struct Plain;

    impl OutputMode for Plain {
        type Wrapped<'life, T> = T;
    }

    pub struct Async;

    impl OutputMode for Async {
        type Wrapped<'life, T> = Pin<Box<dyn Future<Output = T> + Send + 'life>>;
    }

    /// The invocation of a method may fail with the error `E`.
    pub struct Fallible<E>(PhantomData<E>);

    impl<E> OutputMode for Fallible<E> {
        type Wrapped<'life, T> = Result<E, T>;
    }

    pub struct Combined<OO, OI>(OO, OI);

    impl<OO: OutputMode, OI: OutputMode> OutputMode for Combined<OO, OI> {
        type Wrapped<'life, T> = OO::Wrapped<'life, OI::Wrapped<'life, T>>;
    }

    pub type AsyncServer = Async;
    pub type AsyncClient<E> = Combined<Async, Fallible<E>>;
}

pub trait OutputMode {
    type Wrapped<'life, T>;
}

/// Invoke a method with a specific output mode.
pub trait Invoke<M: Method, OM: OutputMode = output::Plain> {
    /// Invoke a method with a specific output mode.
    fn invoke(&self, method: M) -> OM::Wrapped<'_, M::Output>;
}

pub trait InvokeMut<M: Method, OM: OutputMode = output::Plain> {
    fn invoke_mut(&mut self, method: M) -> OM::Wrapped<'_, M::Output>;
}
