use std::error::Error as StdError;
use std::sync::Arc;

use luajit_bindings::{Poppable, Pushable};
use nvim_types::{Array, Function, Object};

macro_rules! impl_into {
    ($trait:ident, $type:ty) => {
        impl $trait for $type {
            fn to_object(self) -> Object {
                self.into()
            }
        }
    };
}

/// A string or an integer.
pub trait StringOrInt {
    fn to_object(self) -> Object;
}

impl_into!(StringOrInt, &str);
impl_into!(StringOrInt, String);
impl_into!(StringOrInt, i8);
impl_into!(StringOrInt, u8);
impl_into!(StringOrInt, i16);
impl_into!(StringOrInt, u16);
impl_into!(StringOrInt, i32);
impl_into!(StringOrInt, u32);
impl_into!(StringOrInt, i64);

/// A string or a list of strings.
pub trait StringOrListOfStrings {
    fn to_object(self) -> Object;
}

impl_into!(StringOrListOfStrings, &str);
impl_into!(StringOrListOfStrings, String);

// Here I'd like to use `IntoIterator` instead of `Vec`, but without
// specilization that'd cause conflicting impls.
impl<S: Into<String>> StringOrListOfStrings for Vec<S> {
    #[inline]
    fn to_object(self) -> Object {
        Array::from_iter(self.into_iter().map(Into::into)).into()
    }
}

/// A Rust closure or a [`Function`].
pub trait ToFunction<A, R> {
    fn to_object(self) -> Object;
}

impl<A, R, F> ToFunction<A, R> for F
where
    A: Poppable,
    R: Pushable,
    F: FnMut(A) -> crate::Result<R> + 'static,
{
    #[inline]
    fn to_object(self) -> Object {
        Function::from_fn_mut(self).into()
    }
}

impl<A, R> ToFunction<A, R> for Function<A, R> {
    #[inline]
    fn to_object(self) -> Object {
        self.into()
    }
}

/// A Rust closure, a [`Function`] or a string.
pub trait StringOrFunction<A, R> {
    fn to_object(self) -> Object;
}

impl<A, R> StringOrFunction<A, R> for &str {
    #[inline]
    fn to_object(self) -> Object {
        self.into()
    }
}

impl<A, R> StringOrFunction<A, R> for String {
    #[inline]
    fn to_object(self) -> Object {
        self.into()
    }
}

impl<A, R, F> StringOrFunction<A, R> for F
where
    A: Poppable,
    R: Pushable,
    F: FnMut(A) -> crate::Result<R> + 'static,
{
    #[inline]
    fn to_object(self) -> Object {
        Function::from_fn_mut(self).into()
    }
}

impl<A, R> StringOrFunction<A, R> for Function<A, R> {
    #[inline]
    fn to_object(self) -> Object {
        self.into()
    }
}

pub trait ToApiError {
    fn to_api_error(self) -> crate::Error;
}

impl<E: Into<Box<dyn StdError + Send + Sync>>> ToApiError for E {
    fn to_api_error(self) -> crate::Error {
        crate::Error::External(Arc::from(self.into()))
    }
}
