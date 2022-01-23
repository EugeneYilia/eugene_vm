use std::cell::RefMut;
use std::ops::Deref;
use std::rc::Rc;

use crate::runtime::thread::Thread;

pub fn put_static(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    let class = Rc::clone(&stack_frame.class);
}