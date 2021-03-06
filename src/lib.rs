//! Utilities for working with <code>type [Fine<T, E>] = (T, [Result<(), E>](`Result`))</code>.
//!
//! This crate for the most part just ports [`Result`]'s API onto [`Fine`]. See [`Fine`] for the full API.
//!
//! Note that any "`and`"-, "`or`" and "`iter`"-style methods that appear on [`Result`] are excluded from the extensions.
//!
//! If you need one of them, or to escalate with [`?`](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html#the-question-mark-operator),
//! first call [`.not_fine()`](`FineExt::not_fine`) to crumple the [`Fine<T, E>`] into a classic [`Result<T, E>`].
//!
//! (More) `transpose` and `flatten` methods are also excluded, unless I figure out what makes the most sense there.
//!
//! ☕🐕

#![doc(html_root_url = "https://docs.rs/this-is-fine/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![no_std]

use core::{
	fmt::Debug,
	ops::{Deref, DerefMut},
};

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

/// Broadly speaking, this is the result of an operation that *sort of* can't fail.
///
/// # Added Methods
///
#[allow(clippy::doc_markdown)]
/// - <code>[.fine()](`FineExt::fine`) -> T</code>
/// - <code>[.not_fine()](`FineExt::not_fine`) -> [Result<T, E>]</code>
#[must_use]
pub type Fine<T, E> = (T, Result<(), E>);

/// Creates a [`Fine`] from a pair of a value and optional error.
pub fn from_inverse<T, E>((value, error): (T, Option<E>)) -> Fine<T, E> {
	(value, error.map_or(Ok(()), Err))
}

pub mod prelude {
	pub use crate::{
		FineExt, FineExtWhereEDebug, FineExtWhereTDebug, FineExtWhereTDeref, FineExtWhereTDerefMut,
		FineTransposeExt,
	};
}

pub trait FineExt<T, E> {
	/// Returns [`true`] iff the [`Result`] is [`Ok(())`](`Ok`).
	#[must_use]
	fn is_ok(&self) -> bool;

	/// Returns [`true`] iff the [`Result`] is [`Err`].
	#[must_use]
	fn is_err(&self) -> bool;

	/// Converts from [`Fine<T, E>`] to [`Option<T>`].
	///
	/// Only [`Some`] if the [`Result`] was [`Ok`].
	fn ok(self) -> Option<T>;

	/// Unwraps the `T`, ignoring any [`Err`].
	fn fine(self) -> T;

	/// Converts from [`Fine<T, E>`] to [`Option<E>`].
	///
	/// Equivalent to [`.1.err()`](`Result::err`).
	#[must_use = "It's unclear whether you meant to discard the error. Prefer `.fine()` if you do."]
	fn err(self) -> Option<E>;

	/// Converts from [`Fine<T, E>`] to [`Result<T, E>`].
	///
	/// # Errors
	///
	/// Iff the [`Result`] was [`Err`], in which case the `T` is discarded.
	fn not_fine(self) -> Result<T, E>;

	/// Converts from [`Fine<T, E>`] to [`Result<T, (T, E)>`].
	///
	/// # Errors
	///
	/// Iff the [`Result`] was [`Err`].
	fn into_result(self) -> Result<T, (T, E)>;

	/// Converts from [`&Fine<T, E>`](`Fine`) to [`Fine<&T, &E>`].
	///
	/// Produces a new [`Fine`], containing one or two references into the original, leaving the original in place.
	fn as_ref(&self) -> Fine<&T, &E>;

	/// Converts from [`&mut Fine<T, E>`](`Fine`) to [`Fine<&mut T, &mut E>`].
	///
	/// Produces a new [`Fine`], containing one or two references into the original, leaving the original in place.
	fn as_mut(&mut self) -> Fine<&mut T, &mut E>;

	/// Maps a [`Fine<T, E>`] to [`Fine<U, E>`],
	/// by *unconditionally* applying a function to the contained `T`,
	/// leaving the [`Result`] untouched.
	fn map<U, F>(self, op: F) -> Fine<U, E>
	where
		F: FnOnce(T) -> U;

	/// Maps a [`Fine<T, E>`] to [`Fine<T, F>`],
	/// by applying a function to a contained [`Err`]'s `E`,
	/// leaving the `T` untouched.
	fn map_err<F, O>(self, op: O) -> Fine<T, F>
	where
		O: FnOnce(E) -> F;
}
impl<T, E> FineExt<T, E> for Fine<T, E> {
	fn is_ok(&self) -> bool {
		self.1.is_ok()
	}

	fn is_err(&self) -> bool {
		self.1.is_err()
	}

	fn ok(self) -> Option<T> {
		self.1.is_ok().then(|| self.0)
	}

	fn fine(self) -> T {
		self.0
	}

	fn err(self) -> Option<E> {
		self.1.err()
	}

	fn not_fine(self) -> Result<T, E> {
		self.1?;
		Ok(self.0)
	}

	fn into_result(self) -> Result<T, (T, E)> {
		match self.1 {
			Ok(()) => Ok(self.0),
			Err(e) => Err((self.0, e)),
		}
	}

	fn as_ref(&self) -> Fine<&T, &E> {
		(&self.0, self.1.as_ref().err().map_or(Ok(()), Err))
	}

	fn as_mut(&mut self) -> Fine<&mut T, &mut E> {
		(&mut self.0, self.1.as_mut().err().map_or(Ok(()), Err))
	}

	fn map<U, F>(self, op: F) -> Fine<U, E>
	where
		F: FnOnce(T) -> U,
	{
		(op(self.0), self.1)
	}

	fn map_err<F, O>(self, op: O) -> Fine<T, F>
	where
		O: FnOnce(E) -> F,
	{
		(self.0, self.1.map_err(op))
	}
}

pub trait FineExtWhereEDebug<T, E>
where
	E: Debug,
{
	/// Unwraps the `T`.
	///
	/// # Panics
	///
	/// Iff the [`Result`] is [`Err`], with a panic message including `msg` and the content of the [`Err`].
	#[track_caller]
	fn expect(self, msg: &str) -> T;

	/// Unwraps the `T`.
	///
	/// # Panics
	///
	/// Iff the [`Result`] is [`Err`], with a panic message provided by the [`Err`]'s value.
	#[track_caller]
	fn unwrap(self) -> T;
}
impl<T, E> FineExtWhereEDebug<T, E> for Fine<T, E>
where
	E: Debug,
{
	#[track_caller]
	fn expect(self, msg: &str) -> T {
		self.1.expect(msg);
		self.0
	}

	#[track_caller]
	fn unwrap(self) -> T {
		self.1.unwrap();
		self.0
	}
}

pub trait FineExtWhereTDebug<T, E>
where
	T: Debug,
{
	/// Unwraps the `E`.
	///
	/// # Panics
	///
	/// Iff the [`Result`] is not [`Err`], with a panic message including `msg` and the `T`.
	#[track_caller]
	fn expect_err(self, msg: &str) -> E;

	/// Unwraps the `E`.
	///
	/// # Panics
	///
	/// Iff the [`Result`] is [`Err`], with a panic message provided by the `T`.
	#[track_caller]
	fn unwrap_err(self) -> E;
}
impl<T, E> FineExtWhereTDebug<T, E> for Fine<T, E>
where
	T: Debug,
{
	#[track_caller]
	fn expect_err(self, msg: &str) -> E {
		self.not_fine().expect_err(msg)
	}

	#[track_caller]
	fn unwrap_err(self) -> E {
		self.not_fine().unwrap_err()
	}
}

pub trait FineExtWhereTDeref<T, E>
where
	T: Deref,
{
	/// Coerces the `T` via [`Deref`] and returns a new [`Fine`] referencing the original.
	fn as_deref(&self) -> Fine<&<T as Deref>::Target, &E>;
}
impl<T, E> FineExtWhereTDeref<T, E> for Fine<T, E>
where
	T: Deref,
{
	fn as_deref(&self) -> Fine<&<T as Deref>::Target, &E> {
		self.as_ref().map(Deref::deref)
	}
}

pub trait FineExtWhereTDerefMut<T, E>
where
	T: DerefMut,
{
	/// Coerces the `T` via [`DerefMut`] and returns a new [`Fine`] referencing the original.
	fn as_deref_mut(&mut self) -> Fine<&mut <T as Deref>::Target, &mut E>;
}
impl<T, E> FineExtWhereTDerefMut<T, E> for Fine<T, E>
where
	T: DerefMut,
{
	fn as_deref_mut(&mut self) -> Fine<&mut <T as Deref>::Target, &mut E> {
		self.as_mut().map(DerefMut::deref_mut)
	}
}

pub trait FineTransposeExt<T, E0, E1> {
	/// Exchanges the [`Result`]s of nested [`Fine`]s.
	fn transpose(self) -> Fine<Fine<T, E1>, E0>;
}
impl<T, E0, E1> FineTransposeExt<T, E0, E1> for Fine<Fine<T, E0>, E1> {
	fn transpose(self) -> Fine<Fine<T, E1>, E0> {
		let ((t, e0), e1) = self;
		((t, e1), e0)
	}
}
