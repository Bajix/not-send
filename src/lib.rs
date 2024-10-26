#![no_std]

use core::marker::PhantomData;

/// Types with PhantomNotSend won't automatically implement Send
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhantomNotSend {
    _marker: PhantomData<*mut ()>,
}

#[allow(non_upper_case_globals)]
pub const PhantomNotSend: PhantomNotSend = PhantomNotSend {
    _marker: PhantomData,
};

unsafe impl Sync for PhantomNotSend {}
