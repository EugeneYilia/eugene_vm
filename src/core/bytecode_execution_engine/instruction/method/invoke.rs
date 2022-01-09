use std::cell::RefMut;
use std::ops::Deref;
use std::rc::Rc;

use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::thread::Thread;

/// 实例方法调用参数收集形式
/// 参数从operand_stack收集的方式  第一个pop出来的是参数n 一直到参数1  之后再是对象引用object_ref

/// Invoke instance method; dispatch based on class
pub fn invoke_virtual(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let class = Rc::clone(&stack_frame.class);
    let virtual_method_index = stack_frame.code_reader.read_u16() as usize;
    if let ConstantInfo::MethodRef { class_index, name_and_type_index } = class.constant_pool.get(virtual_method_index) {} else {}

    let value = stack_frame.operand_stack.pop_i32();
    info!("{}", value);
    // println!("{}", value);
}

/// special handling for superclass, private, and instance initialization method invocations
pub fn invoke_special(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let class = Rc::clone(&stack_frame.class);
    let special_method_index = stack_frame.code_reader.read_u16() as usize;
    if let ConstantInfo::MethodRef { class_index, name_and_type_index } = class.constant_pool.get(special_method_index) {} else {}
}