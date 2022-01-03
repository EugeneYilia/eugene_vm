use std::cell::RefMut;
use std::num::Wrapping;
use std::ops::Deref;

use crate::runtime::thread::Thread;

pub fn fcmpl(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let second = stack_frame.operand_stack.pop_f32();
    let first = stack_frame.operand_stack.pop_f32();
    if first > second {
        stack_frame.operand_stack.push_i32(Wrapping(1i32));
    } else if first == second {
        stack_frame.operand_stack.push_i32(Wrapping(0i32));
    } else if first < second {
        stack_frame.operand_stack.push_i32(Wrapping(-1i32));
    } else {
        // 其中有一个数值为NAN时 将-1压入栈顶
        stack_frame.operand_stack.push_i32(Wrapping(-1i32));
    }

}

pub fn fcmpg(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let second = stack_frame.operand_stack.pop_f32();
    let first = stack_frame.operand_stack.pop_f32();
    if first > second {
        stack_frame.operand_stack.push_i32(Wrapping(1i32));
    } else if first == second {
        stack_frame.operand_stack.push_i32(Wrapping(0i32));
    } else if first < second {
        stack_frame.operand_stack.push_i32(Wrapping(-1i32));
    } else {
        // 其中有一个数值为NAN时 将1压入栈顶
        stack_frame.operand_stack.push_i32(Wrapping(1i32));
    }


}

#[test]
fn test_compare_with_nan() {
    // 和NAN比较结果都为false
    println!("{}", 1.0 > f32::NAN);
    println!("{}", 1.0 < f32::NAN);
    println!("{}", 1.0 == f32::NAN);
}