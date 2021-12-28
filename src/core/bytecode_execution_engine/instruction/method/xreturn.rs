use std::cell::RefMut;

use crate::runtime::thread::Thread;

/// return
pub fn r#return(thread: &mut RefMut<Thread>) {
    thread.pop_stack_frame();
}

