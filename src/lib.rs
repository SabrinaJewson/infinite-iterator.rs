//! Iterators that never end.
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

use core::iter;

/// An [`Iterator`] that never ends.
///
/// # Invariants
///
/// For this trait to be correctly implemented,
/// the following invariants must be upheld:
/// 1. `Some(iter.next_infinite())` must always give the same result as `iter.next()`.
/// 2. No default-implemented iterator methods may be overriden
///     to have visibly different behaviour
///     than their default implementations.
/// 3. `size_hint().1` must always be `None`.
/// 4. The type must not implement [`ExactSizeIterator`].
///
/// If any of the above invariants are violated,
/// the behaviour of any code that uses the iterator is unspecified
/// (i.e. it may panic, abort or give wrong results) —
/// however, because `InfiniteIterator` is not an `unsafe trait`
/// it still must not invoke Undefined Behaviour.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait InfiniteIterator: Iterator {
    /// Like [`Iterator::next`],
    /// but never returning [`None`] because the iterator never ends.
    fn next_infinite(&mut self) -> Self::Item;
}

impl<I: ?Sized + InfiniteIterator> InfiniteIterator for &mut I {
    fn next_infinite(&mut self) -> Self::Item {
        (**self).next_infinite()
    }
}

#[cfg(feature = "alloc")]
impl<I: ?Sized + InfiniteIterator> InfiniteIterator for alloc::boxed::Box<I> {
    fn next_infinite(&mut self) -> Self::Item {
        (**self).next_infinite()
    }
}

impl<'item, I, T> InfiniteIterator for iter::Cloned<I>
where
    T: 'item + Clone,
    I: InfiniteIterator<Item = &'item T>,
{
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<'item, I, T> InfiniteIterator for iter::Copied<I>
where
    T: 'item + Copy,
    I: InfiniteIterator<Item = &'item T>,
{
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<A: Clone> InfiniteIterator for iter::Repeat<A> {
    fn next_infinite(&mut self) -> Self::Item {
        // SAFETY: `Repeat` never ends.
        unsafe { self.next().unwrap_unchecked() }
    }
}

impl<F: FnMut() -> A, A> InfiniteIterator for iter::RepeatWith<F> {
    fn next_infinite(&mut self) -> Self::Item {
        unsafe { self.next().unwrap_unchecked() }
    }
}

impl<A, B> InfiniteIterator for iter::Chain<A, B>
where
    A: Iterator,
    B: InfiniteIterator<Item = A::Item>,
{
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<A, B> InfiniteIterator for iter::Zip<A, B>
where
    A: InfiniteIterator,
    B: InfiniteIterator,
{
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<I, P> InfiniteIterator for iter::Filter<I, P>
where
    I: InfiniteIterator,
    P: FnMut(&I::Item) -> bool,
{
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<B, I, F> InfiniteIterator for iter::FilterMap<I, F>
where
    I: InfiniteIterator,
    F: FnMut(I::Item) -> Option<B>,
{
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<B, I, F> InfiniteIterator for iter::Map<I, F>
where
    I: InfiniteIterator,
    F: FnMut(I::Item) -> Option<B>,
{
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

// Require `InfiniteIterator` to prevent empty iterators
impl<I: Clone + InfiniteIterator> InfiniteIterator for iter::Cycle<I> {
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<I: InfiniteIterator> InfiniteIterator for iter::Enumerate<I> {
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<I: InfiniteIterator> InfiniteIterator for iter::Fuse<I> {
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<I: InfiniteIterator> InfiniteIterator for iter::Peekable<I> {
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

/// An extension trait providing extra methods to [`iter::Peekable`]
/// when the underlying iterator never ends.
///
/// This trait is sealed;
/// it can only be implemented on [`iter::Peekable`].
pub trait PeekableExt: peekable_ext::Sealed {
    /// Like [`iter::Peekable::peek`],
    /// but always returning a reference
    /// because the underlying iterator never ends.
    fn peek_infinite(&mut self) -> &Self::Item;

    /// Like [`iter::Peekable::peek_mut`],
    /// but always returning a unique reference
    /// because the underlying iterator never ends.
    fn peek_infinite_mut(&mut self) -> &mut Self::Item;
}

mod peekable_ext {
    pub trait Sealed: Sized + Iterator {}
}

impl<I: InfiniteIterator> peekable_ext::Sealed for iter::Peekable<I> {}
impl<I: InfiniteIterator> PeekableExt for iter::Peekable<I> {
    fn peek_infinite(&mut self) -> &Self::Item {
        self.peek().unwrap()
    }

    fn peek_infinite_mut(&mut self) -> &mut Self::Item {
        self.peek_mut().unwrap()
    }
}

impl<I: InfiniteIterator> InfiniteIterator for iter::Skip<I> {
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<I, P> InfiniteIterator for iter::SkipWhile<I, P>
where
    I: InfiniteIterator,
    P: FnMut(&I::Item) -> bool,
{
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<I: InfiniteIterator> InfiniteIterator for iter::StepBy<I> {
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<I: InfiniteIterator, F> InfiniteIterator for iter::Inspect<I, F>
where
    I: InfiniteIterator,
    F: FnMut(&I::Item),
{
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<I> InfiniteIterator for iter::Flatten<I>
where
    I: InfiniteIterator,
    I::Item: IntoIterator,
{
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<I, U, F> InfiniteIterator for iter::FlatMap<I, U, F>
where
    I: InfiniteIterator,
    U: IntoIterator,
    F: FnMut(I::Item) -> U,
{
    fn next_infinite(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<A> InfiniteIterator for core::ops::RangeFrom<A>
where
    core::ops::RangeFrom<A>: Iterator,
{
    fn next_infinite(&mut self) -> Self::Item {
        // SAFETY: The `RangeFrom` iterator never ends
        unsafe { self.next().unwrap_unchecked() }
    }
}

#[cfg(feature = "std")]
impl InfiniteIterator for std::net::Incoming<'_> {
    fn next_infinite(&mut self) -> Self::Item {
        // SAFETY: The Incoming iterator never ends.
        unsafe { self.next().unwrap_unchecked() }
    }
}

#[cfg(all(feature = "std", unix))]
impl InfiniteIterator for std::os::unix::net::Incoming<'_> {
    fn next_infinite(&mut self) -> Self::Item {
        // SAFETY: The Incoming iterator never ends
        unsafe { self.next().unwrap_unchecked() }
    }
}
