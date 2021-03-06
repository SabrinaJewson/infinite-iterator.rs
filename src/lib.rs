//! This crate provides a trait,
//! `InfiniteIterator`,
//! used to represent an iterator for which [`next`]
//! can never return `None`.
//!
//! It additionally provides a macro, `ifor!`,
//! which is identical a for loop
//! except it supports breaking with a value
//! when used on an infinite iterator.
//!
//! [`next`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#tymethod.next
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

    /// Like [`Iterator::for_each`],
    /// but it never returns because the iterator never ends.
    ///
    /// # Examples
    ///
    /// ```
    /// use infinite_iterator::InfiniteIterator;
    ///
    /// fn run() -> ! {
    ///     (0..).for_each_infinite(|num| {
    ///         println!("{num}");
    ///         std::thread::sleep(std::time::Duration::from_secs(5));
    ///     })
    /// }
    /// ```
    fn for_each_infinite<F: FnMut(Self::Item)>(mut self, mut f: F) -> !
    where
        Self: Sized,
    {
        loop {
            f(self.next_infinite());
        }
    }

    /// Like [`Iterator::find`],
    /// but it is guaranteed to find an item
    /// (or loop forever)
    /// because the iterator is infinite.
    ///
    /// # Examples
    ///
    /// ```
    /// use infinite_iterator::InfiniteIterator;
    ///
    /// assert_eq!((5..).find_infinite(|&num| num > 10), 11);
    /// ```
    fn find_infinite<P>(&mut self, mut predicate: P) -> Self::Item
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        loop {
            let item = self.next_infinite();
            if predicate(&item) {
                break item;
            }
        }
    }

    /// Like [`Iterator::find_map`],
    /// but it is guaranteed to find an item
    /// (or loop forever)
    /// because the iterator is infinite.
    ///
    /// # Examples
    ///
    /// ```
    /// use infinite_iterator::InfiniteIterator;
    ///
    /// assert_eq!((5_u32..).step_by(3).find_map_infinite(|num| num.checked_sub(10)), 1);
    /// ```
    fn find_map_infinite<B, F>(&mut self, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>,
    {
        loop {
            if let Some(mapped) = f(self.next_infinite()) {
                break mapped;
            }
        }
    }

    /// Like [`Iterator::position`],
    /// but it is guaranteed to find an item
    /// (or loop forever)
    /// because the iterator is infinite.
    ///
    /// # Examples
    ///
    /// ```
    /// use infinite_iterator::InfiniteIterator;
    ///
    /// assert_eq!((5..).position_infinite(|num| num > 10), 6);
    /// ```
    fn position_infinite<P>(&mut self, mut predicate: P) -> usize
    where
        Self: Sized,
        P: FnMut(Self::Item) -> bool,
    {
        let mut i = 0;
        loop {
            if predicate(self.next_infinite()) {
                break i;
            }
            i += 1;
        }
    }
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

/// An extension of `for in` loops with better support for infinite iterators.
///
/// This macro presents a _superset_ of regular `for` loops:
/// it works both with finite and infinite iterators.
///
/// # Examples
///
/// Use with a finite iterator:
///
/// ```
/// use infinite_iterator::ifor;
///
/// ifor!(item in [1, 2, 3] {
///     println!("{item}");
/// });
/// ```
///
/// Use with an infinite iterator:
///
/// ```
/// use infinite_iterator::ifor;
///
/// # fn run() -> ! {
/// ifor!(item in 0.. {
///     println!("{item}");
///     std::thread::sleep(std::time::Duration::from_secs(5));
/// })
/// # }
/// ```
///
/// Infinite iterators additionally support breaking with a value:
///
/// ```
/// use infinite_iterator::ifor;
///
/// let item = ifor!(item in 0.. {
///     if item > 10 {
///         break item;
///     }
/// });
///
/// assert_eq!(item, 11);
/// ```
///
/// You can use loop labels with an `ifor!` too,
/// as long as you write out the keyword `for`:
///
/// ```
/// use infinite_iterator::ifor;
///
/// ifor!('outer: for a in 0..10 {
///     ifor!('inner: for b in 0..10 {
///         println!("{a}, {b}");
///         if a + b > 16 {
///             break 'outer;
///         }
///     });
/// });
/// ```
#[macro_export]
macro_rules! ifor {
    ($($label:lifetime:)? for $pat:pat in $($rest:tt)*) => {
        $crate::__ifor_inner!($($label:)? for $pat in () $($rest)*)
    };
    ($pat:pat in $($rest:tt)*) => {
        $crate::__ifor_inner!(for $pat in () $($rest)*)
    };
}

// Not public API.
#[doc(hidden)]
#[macro_export]
macro_rules! __ifor_inner {
    ($($label:lifetime:)? for $pat:pat in ($expr:expr) $block:block) => {
        match $crate::__private::IntoIterator::into_iter($expr) {
            iter => {
                let mut iter = $crate::__private::MaybeInfinite(iter);
                $($label:)? loop {
                    let $pat = {
                        use $crate::__private::TryNextFallback;
                        match iter.try_next() {
                            $crate::__private::Ok(item) => item,
                            $crate::__private::Err(breakable) => {
                                break breakable.into_break()
                            },
                        }
                    };
                    $block
                }
            }
        }
    };
    ($($label:lifetime:)? for $pat:pat in () $block:block) => {
        $crate::__private::compile_error!("no expression provided to `ifor!`")
    };
    ($($label:lifetime:)? for $pat:pat in ($($expr:tt)*) $first:tt $($rest:tt)*) => {
        $crate::__ifor_inner!($($label:)? for $pat in ($($expr)* $first) $($rest)*)
    };
}

// Not public API.
#[doc(hidden)]
pub mod __private {
    use crate::InfiniteIterator;

    pub use core::compile_error;
    pub use Err;
    pub use IntoIterator;
    pub use Ok;

    pub struct MaybeInfinite<I>(pub I);

    impl<I: InfiniteIterator> MaybeInfinite<I> {
        pub fn try_next(&mut self) -> Result<I::Item, NeverBreak> {
            Ok(self.0.next_infinite())
        }
    }

    pub trait TryNextFallback: Sized {
        type Item;
        fn try_next(&mut self) -> Result<Self::Item, CanBreak>;
    }
    impl<I: Iterator> TryNextFallback for MaybeInfinite<I> {
        type Item = I::Item;
        fn try_next(&mut self) -> Result<Self::Item, CanBreak> {
            self.0.next().ok_or(CanBreak)
        }
    }

    pub enum NeverBreak {}
    impl NeverBreak {
        pub fn into_break(self) -> ! {
            match self {}
        }
    }

    pub struct CanBreak;
    impl CanBreak {
        pub fn into_break(self) {}
    }
}
