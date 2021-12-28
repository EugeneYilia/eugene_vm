use std::cell::RefMut;
use std::num::Wrapping;

use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn fcmpl(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let second = operand_stack.pop_f32();
    let first = operand_stack.pop_f32();
    if first > second {
        operand_stack.push_i32(Wrapping(1i32));
    } else if first == second {
        operand_stack.push_i32(Wrapping(0i32));
    } else if first < second {
        operand_stack.push_i32(Wrapping(-1i32));
    } else {
        // 其中有一个数值为NAN时 将-1压入栈顶
        operand_stack.push_i32(Wrapping(-1i32));
    }

}

pub fn fcmpg(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let second = operand_stack.pop_f32();
    let first = operand_stack.pop_f32();
    if first > second {
        operand_stack.push_i32(Wrapping(1i32));
    } else if first == second {
        operand_stack.push_i32(Wrapping(0i32));
    } else if first < second {
        operand_stack.push_i32(Wrapping(-1i32));
    } else {
        // 其中有一个数值为NAN时 将1压入栈顶
        operand_stack.push_i32(Wrapping(1i32));
    }


}

#[test]
fn test_compare_with_nan() {
    // 和NAN比较结果都为false
    println!("{}", 1.0 > f32::NAN);
    println!("{}", 1.0 < f32::NAN);
    println!("{}", 1.0 == f32::NAN);
}