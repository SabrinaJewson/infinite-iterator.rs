#![no_implicit_prelude]
#![warn(clippy::all, clippy::pedantic, unused_lifetimes, unused_qualifications)]

#[test]
fn hygiene() {
    ::infinite_iterator::ifor!(x in 0_u32..10 {
        ::std::println!("{x}");
    });
}
