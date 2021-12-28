use std::cell::RefMut;

use crate::runtime::thread::Thread;

pub fn nop(_thread: &mut RefMut<Thread>) {}