use std::cell::RefMut;
use std::num::Wrapping;

use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;
use crate::util::instruction_util::variable_slot_type_is_kind_one;

// oracle instruction doc
// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-6.html

pub fn dup(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let variable_slot_clone = operand_stack.last().clone();
    if variable_slot_type_is_kind_one(&variable_slot_clone) {
        operand_stack.push(variable_slot_clone);
    } else {
        panic!("dup error:  variable_slot is {:?}", variable_slot_clone);
    }


}

/// 复制栈顶数据并将复制的数据插入到栈顶第二个元素之下
pub fn dup_x1(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop();
    let second = operand_stack.pop();
    let first_clone = first.clone();
    // stack bottom [x,y,second,first] stack head
    // stack bottom [x,y,first_clone,second,first] stack head
    if variable_slot_type_is_kind_one(&first) && variable_slot_type_is_kind_one(&second) {
        operand_stack.extend_with_slice(&[first_clone, second, first]);
    } else {
        panic!("dup_x1 error:  variable_slot first is {:?}   variable_slot second is {:?}", first, second);
    }


}

pub fn dup_x2(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop();
    let second = operand_stack.pop();
    if variable_slot_type_is_kind_one(&first) {
        if variable_slot_type_is_kind_one(&second) {
            let third = operand_stack.pop();
            if variable_slot_type_is_kind_one(&third) {
                operand_stack.extend_with_slice(&[first.clone(), third, second, first]);
            } else {
                panic!("dup_x2 error:  variable_slot third is {:?}", third);
            }
        } else {
            operand_stack.extend_with_slice(&[first.clone(), second, first]);
        }
    } else {
        panic!("dup_x2 error:  variable_slot first is {:?}", first);
    }

}

pub fn dup2(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop();
    if variable_slot_type_is_kind_one(&first) {
        let second = operand_stack.pop();
        if variable_slot_type_is_kind_one(&second) {
            operand_stack.extend_with_slice(&[second.clone(), first.clone(), second, first]);
        } else {
            panic!("dup2 error:  variable_slot second is {:?}", second);
        }
    } else {
        operand_stack.extend_with_slice(&[first.clone(), first]);
    }
}

pub fn dup2_x1(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop();
    let second = operand_stack.pop();
    if variable_slot_type_is_kind_one(&first) {
        // first second third are all category 1
        if variable_slot_type_is_kind_one(&second) {
            let third = operand_stack.pop();
            if variable_slot_type_is_kind_one(&third) {
                operand_stack.extend_with_slice(&[second.clone(), first.clone(), third, second, first]);
            } else {
                panic!("dup2_x1 error:  variable_slot third is {:?}", third);
            }
        } else {
            panic!("dup2_x1 error:  variable_slot second is {:?}", second);
        }
    } else {
        // first category 2
        // second category 1
        if variable_slot_type_is_kind_one(&second) {
            operand_stack.extend_with_slice(&[first.clone(), second, first]);
        } else {
            panic!("dup2_x1 error:  variable_slot second is {:?}", second);
        }
    }
}

pub fn dup2_x2(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop();
    let second = operand_stack.pop();
    if variable_slot_type_is_kind_one(&first) {
        if variable_slot_type_is_kind_one(&second) {
            let third = operand_stack.pop();
            if variable_slot_type_is_kind_one(&third) {
                let fourth = operand_stack.pop();
                if variable_slot_type_is_kind_one(&fourth) {
                    // stack head 1 1 1 1 stack bottom
                    operand_stack.extend_with_slice(&[second.clone(), first.clone(), fourth, third, second, first]);
                } else {
                    panic!("dup2_x2 error:  variable_slot fourth is {:?}", fourth);
                }
            } else {
                // stack head 1 1 2 tack bottom
                operand_stack.extend_with_slice(&[second.clone(), first.clone(), third, second, first]);
            }
        } else {
            panic!("dup2_x2 error:  variable_slot second is {:?}", second);
        }
    } else {
        if variable_slot_type_is_kind_one(&second) {
            // stack head 2 1 1 stack bottom
            let third = operand_stack.pop();
            if variable_slot_type_is_kind_one(&third) {
                operand_stack.extend_with_slice(&[first.clone(), third, second, first]);
            } else {
                panic!("dup2_x2 error:  variable_slot third is {:?}", third);
            }
        } else {
            // stack head 2 2 stack bottom
            operand_stack.extend_with_slice(&[first.clone(), second, first]);
        }
    }
}

#[test]
fn test_wrapping() {
    let a = Wrapping(i32::MAX);
    let b = Wrapping(i32::MAX);
    let c = a + b;
    println!("{}", c);

    let d = Wrapping(i64::MAX);
    let e = Wrapping(i64::MAX);
    let f = d + e;
    println!("{}", f);

    let g = Wrapping(3);
    let h = Wrapping(4);
    println!("{}", g > h);
    println!("{}", g <= h);
    println!("{}", g + h);

    let z = f32::MAX;
    let x = f32::MAX;
    let q = z + x;
    println!("{}", q);
}

#[test]
fn add_elements_to_vec() {
    let mut elements = vec![1, 2, 3];
    println!("{:?}", elements);
    elements.extend_from_slice(&[4, 5, 6]);
    println!("{:?}", elements);
}