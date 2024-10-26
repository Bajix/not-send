#![cfg_attr(not(test), no_std)]

use core::marker::PhantomData;

#[allow(rustdoc::invalid_rust_codeblocks)]
/// Types with PhantomNotSend won't automatically implement Send
///
/// # Example
///
/// ```compile_fail
/// let not_send = not_send::PhantomNotSend;
/// std::thread::spawn(move || {
///     drop(not_send);
/// }).join().unwrap();
/// ```
///
/// ```ignore
/// error[E0277]: `*mut ()` cannot be sent between threads safely
///    --> src/lib.rs:13:20
///     |
/// 6   |   std::thread::spawn(move || {
///     |   ------------------ ^------
///     |   |                  |
///     |  _|__________________within this `{closure@src/lib.rs:6:20: 6:27}`
///     | | |
///     | | required by a bound introduced by this call
/// 7   | |     drop(not_send);
/// 8   | | }).join().unwrap();
///     | |_^ `*mut ()` cannot be sent between threads safely
///     |
///     = help: within `{closure@src/lib.rs:6:20: 6:27}`, the trait `Send` is not implemented for `*mut ()`, which is required by `{closure@src/lib.rs:6:20: 6:27}: Send`
/// note: required because it appears within the type `PhantomData<*mut ()>`
///    --> /Users/bajix/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/marker.rs:753:12
///     |
/// 753 | pub struct PhantomData<T: ?Sized>;
///     |            ^^^^^^^^^^^
/// note: required because it appears within the type `PhantomNotSend`
///    --> /Users/bajix/Projects/not-send/src/lib.rs:57:12
///     |
/// 57  | pub struct PhantomNotSend {
///     |            ^^^^^^^^^^^^^^
/// note: required because it's used within this closure
///    --> src/lib.rs:13:20
///     |
/// 6   | std::thread::spawn(move || {
///     |                    ^^^^^^^
/// note: required by a bound in `spawn`
///    --> /Users/bajix/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/mod.rs:707:8
///     |
/// 704 | pub fn spawn<F, T>(f: F) -> JoinHandle<T>
///     |        ----- required by a bound in this function
/// ...
/// 707 |     F: Send + 'static,
///     |        ^^^^ required by this bound in `spawn`
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhantomNotSend {
    _marker: PhantomData<*mut ()>,
}

#[allow(non_upper_case_globals)]
pub const PhantomNotSend: PhantomNotSend = PhantomNotSend {
    _marker: PhantomData,
};

unsafe impl Sync for PhantomNotSend {}
