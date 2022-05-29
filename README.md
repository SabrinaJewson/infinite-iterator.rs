# infinite-iterator

This crate provides a trait,
`InfiniteIterator`,
used to represent an iterator for which [`next`]
can never return `None`.

It additionally provides a macro, `ifor!`,
which is identical a for loop
except it supports breaking with a value
when used on an infinite iterator.

[`next`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#tymethod.next

License: MIT
