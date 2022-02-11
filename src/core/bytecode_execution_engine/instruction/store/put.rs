use std::cell::RefMut;
use std::ops::Deref;
use std::rc::Rc;

use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::thread::Thread;

pub fn put_static(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    let class = Rc::clone(&stack_frame.class);

    let field_index = stack_frame.code_reader.read_u16();
    if let ConstantInfo::FieldRef { class_index, name_and_type_index } = class.constant_pool.get(field_index as usize) {}
}