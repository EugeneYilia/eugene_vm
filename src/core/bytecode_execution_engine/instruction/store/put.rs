use std::cell::RefMut;

use crate::runtime::thread::Thread;

pub fn put_static(thread: &mut RefMut<Thread>) {}