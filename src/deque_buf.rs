// Copyright 2018 Austin Bonander <austin.bonander@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms
//! Move-free buffer and reader utilizing the [`slice-deque`] crate.
//!
//! These types are only available on target platforms with virtual memory support,
//! namely Windows, OS X and Linux.
//!
//! [`slice-deque`]: https://crates.io/crates/slice-deque
extern crate slice_deque;
use self::slice_deque::SliceDeque;

pub struct BufImpl {
    deque: SliceDeque<u8>,
}

impl BufImpl {
    pub fn with_capacity(cap: usize) -> Self {
        BufImpl {
            deque: SliceDeque::with_capacity(cap),
        }
    }

    pub fn capacity(&self) -> usize {
        self.deque.capacity()
    }

    pub fn len(&self) -> usize { self.deque.len() }

    pub fn usable_space(&self) -> usize {
        self.capacity() - self.len()
    }

    pub fn reserve(&mut self, additional: usize) -> bool {
        self.deque.reserve(additional);
        true
    }

    pub fn make_room(&mut self) {
        // no-op
    }

    pub fn buf(&self) -> &[u8] { &self.deque }

    pub fn buf_mut(&mut self) -> &mut [u8] { &mut self.deque }

    pub unsafe fn write_buf(&mut self) -> &mut [u8] {
        self.deque.tail_head_slice()
    }

    pub unsafe fn bytes_written(&mut self, add: usize) {
        let offset = add as isize;

        if offset < 0 {
            panic!("BufImpl.bytes_written() arg overflowed isize");
        }

        self.deque.move_tail(offset);
    }

    pub fn consume(&mut self, amt: usize) {
        self.deque.truncate_front(amt);
    }
}