use super::Loader;
use crate::{BorrowWithNamespace, DisplayWithNamespace, IriNamespace};
use futures::future::{BoxFuture, FutureExt};
use locspan::Meta;
use std::fmt;
use std::marker::PhantomData;

/// Dummy loader.
///
/// A dummy loader that does not load anything.
/// Can be useful when you know that you will never need to load remote resource.
///
/// Raises an `LoadingDocumentFailed` at every attempt to load a resource.
pub struct NoLoader<I, T, M>(PhantomData<(I, T, M)>);

#[derive(Debug)]
pub struct CannotLoad<I>(I);

impl<I: fmt::Display> fmt::Display for CannotLoad<I> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "cannot load `{}`", self.0)
	}
}

impl<I: DisplayWithNamespace<N>, N> DisplayWithNamespace<N> for CannotLoad<I> {
	fn fmt_with(&self, namespace: &N, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "cannot load `{}`", self.0.with_namespace(namespace))
	}
}

impl<I, T, M> NoLoader<I, T, M> {
	#[inline(always)]
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<I, T, M> Default for NoLoader<I, T, M> {
	#[inline(always)]
	fn default() -> Self {
		Self::new()
	}
}

impl<I: Send, T, M> Loader<I> for NoLoader<I, T, M> {
	type Output = T;
	type Error = CannotLoad<I>;
	type Metadata = M;

	#[inline(always)]
	fn load_in<'a>(
		&'a mut self,
		_namespace: &impl IriNamespace<I>,
		url: I,
	) -> BoxFuture<'a, Result<Meta<T, M>, Self::Error>>
	where
		I: 'a,
	{
		async move { Err(CannotLoad(url)) }.boxed()
	}
}