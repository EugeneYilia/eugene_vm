use std::cell::RefMut;
use std::num::Wrapping;
use std::ops::Deref;
use std::rc::Rc;

use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::thread::Thread;

/// 将int, float或String型常量值从常量池中推送至栈顶
pub fn ldc(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    let constant_pool_index = stack_frame.code_reader.read_u8();
    let class_ref = Rc::clone(&stack_frame.class);
    let constant_info = class_ref.constant_pool.get(constant_pool_index as usize);
    match constant_info {
        ConstantInfo::Integer(value) => stack_frame.operand_stack.push_i32(Wrapping(*value)),
        ConstantInfo::Float(value) => stack_frame.operand_stack.push_f32(*value),
        // TODO: 使用更好的方式将String的ref推送到操作数栈上
        ConstantInfo::String(value) => stack_frame.operand_stack.push_i32(Wrapping(*value as i32)),
        _ => panic!("Class Format Error: {:?}", Rc::clone(&stack_frame.class))
    }


}

/// 将int, float或String型常量值从常量池中推送至栈顶(宽索引)
pub fn ldc_w(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    // 宽索引 u16
    let constant_pool_index = stack_frame.code_reader.read_u16();
    let class_ref = Rc::clone(&stack_frame.class);
    let constant_info = class_ref.constant_pool.get(constant_pool_index as usize);
    match constant_info {
        ConstantInfo::Integer(value) => stack_frame.operand_stack.push_i32(Wrapping(*value)),
        ConstantInfo::Float(value) => stack_frame.operand_stack.push_f32(*value),
        // TODO: 使用更好的方式将String的ref推送到操作数栈上
        ConstantInfo::String(value) => stack_frame.operand_stack.push_i32(Wrapping(*value as i32)),
        _ => panic!("Class Format Error: {:?}", Rc::clone(&stack_frame.class))
    }

}

/// 将long或double型常量值从常量池中推送至栈顶(宽索引)
pub fn ldc2_w(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    // 宽索引 u16
    let constant_pool_index = stack_frame.code_reader.read_u16();
    let class_ref = Rc::clone(&stack_frame.class);
    let constant_info = class_ref.constant_pool.get(constant_pool_index as usize);
    match constant_info {
        ConstantInfo::Long(value) => stack_frame.operand_stack.push_i64(Wrapping(*value)),
        ConstantInfo::Double(value) => stack_frame.operand_stack.push_f64(*value),
        _ => panic!("Class Format Error: {:?}", Rc::clone(&stack_frame.class))
    }
}
