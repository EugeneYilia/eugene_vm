use std::cell::RefMut;
use std::num::Wrapping;

use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

/// 将int, float或String型常量值从常量池中推送至栈顶
pub fn ldc(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, class, .. } = stack_frame;
    let constant_pool_index = code_reader.read_u8();
    let class_ref = class.clone();
    let constant_info = class_ref.constant_pool.get(constant_pool_index as usize);
    match constant_info {
        ConstantInfo::Integer(value) => operand_stack.push_i32(Wrapping(*value)),
        ConstantInfo::Float(value) => operand_stack.push_f32(*value),
        // TODO: 使用更好的方式将String的ref推送到操作数栈上
        ConstantInfo::String(value) => operand_stack.push_i32(Wrapping(*value as i32)),
        _ => panic!("Class Format Error: {:?}", class.clone())
    }

    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

/// 将int, float或String型常量值从常量池中推送至栈顶(宽索引)
pub fn ldc_w(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, class, .. } = stack_frame;
    // 宽索引 u16
    let constant_pool_index = code_reader.read_u16();
    let class_ref = class.clone();
    let constant_info = class_ref.constant_pool.get(constant_pool_index as usize);
    match constant_info {
        ConstantInfo::Integer(value) => operand_stack.push_i32(Wrapping(*value)),
        ConstantInfo::Float(value) => operand_stack.push_f32(*value),
        // TODO: 使用更好的方式将String的ref推送到操作数栈上
        ConstantInfo::String(value) => operand_stack.push_i32(Wrapping(*value as i32)),
        _ => panic!("Class Format Error: {:?}", class.clone())
    }
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

/// 将long或double型常量值从常量池中推送至栈顶(宽索引)
pub fn ldc2_w(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, class, .. } = stack_frame;
    // 宽索引 u16
    let constant_pool_index = code_reader.read_u16();
    let class_ref = class.clone();
    let constant_info = class_ref.constant_pool.get(constant_pool_index as usize);
    match constant_info {
        ConstantInfo::Long(value) => operand_stack.push_i64(Wrapping(*value)),
        ConstantInfo::Double(value) => operand_stack.push_f64(*value),
        _ => panic!("Class Format Error: {:?}", class.clone())
    }
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    use crate::core::bytecode_execution_engine::instruction::load::constant::ldc::{ldc, ldc2_w, ldc_w};
    use crate::core::bytecode_execution_engine::instruction::tests::{mock_class, mock_rc_method};
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
    use crate::runtime::stack::stack_frame::StackFrame;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_ldc_i32() {
        let mut class = mock_class();
        class.constant_pool.insert(1usize, ConstantInfo::Integer(20i32));
        let stack_frame = StackFrame::new(Rc::new(class), mock_rc_method());
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = ldc(&mut CodeReader::new(Rc::new(vec![2u8, 1u8]), 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, 20i32);
        assert_eq!(instruction_execute_result.new_pc, 2usize);
    }

    #[test]
    fn test_ldc_f32() {
        let mut class = mock_class();
        class.constant_pool.insert(1usize, ConstantInfo::Float(2.1f32));
        let stack_frame = StackFrame::new(Rc::new(class), mock_rc_method());
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = ldc(&mut CodeReader::new(Rc::new(vec![2u8, 1u8]), 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_f32();
        assert_eq!(result, 2.1f32);
        assert_eq!(instruction_execute_result.new_pc, 2usize);
    }

    #[test]
    fn test_ldc_string() {
        let mut class = mock_class();
        class.constant_pool.insert(1usize, ConstantInfo::String(17u16));
        let stack_frame = StackFrame::new(Rc::new(class), mock_rc_method());
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = ldc(&mut CodeReader::new(Rc::new(vec![2u8, 1u8]), 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, 17i32);
        assert_eq!(instruction_execute_result.new_pc, 2usize);
    }

    #[test]
    fn test_ldc_w_i32() {
        let mut class = mock_class();
        class.constant_pool.insert(257usize, ConstantInfo::Integer(32i32));
        let stack_frame = StackFrame::new(Rc::new(class), mock_rc_method());
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = ldc_w(&mut CodeReader::new(Rc::new(vec![2u8, 1u8, 1u8]), 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, 32i32);
        assert_eq!(instruction_execute_result.new_pc, 3usize);
    }

    #[test]
    fn test_ldc_w_f32() {
        let mut class = mock_class();
        class.constant_pool.insert(258usize, ConstantInfo::Float(23.1f32));
        let stack_frame = StackFrame::new(Rc::new(class), mock_rc_method());
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = ldc_w(&mut CodeReader::new(Rc::new(vec![2u8, 1u8, 2u8]), 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_f32();
        assert_eq!(result, 23.1f32);
        assert_eq!(instruction_execute_result.new_pc, 3usize);
    }

    #[test]
    fn test_ldc_w_string() {
        let mut class = mock_class();
        class.constant_pool.insert(259usize, ConstantInfo::String(66u16));
        let stack_frame = StackFrame::new(Rc::new(class), mock_rc_method());
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = ldc_w(&mut CodeReader::new(Rc::new(vec![2u8, 1u8, 3u8]), 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, 66i32);
        assert_eq!(instruction_execute_result.new_pc, 3usize);
    }

    #[test]
    fn test_ldc2_w_long() {
        let mut class = mock_class();
        class.constant_pool.insert(260usize, ConstantInfo::Long(999i64));
        let stack_frame = StackFrame::new(Rc::new(class), mock_rc_method());
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = ldc2_w(&mut CodeReader::new(Rc::new(vec![2u8, 1u8, 4u8]), 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_i64();
        assert_eq!(result.0, 999i64);
        assert_eq!(instruction_execute_result.new_pc, 3usize);
    }

    #[test]
    fn test_ldc2_w_double() {
        let mut class = mock_class();
        class.constant_pool.insert(261usize, ConstantInfo::Double(66.66f64));
        let stack_frame = StackFrame::new(Rc::new(class), mock_rc_method());
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = ldc2_w(&mut CodeReader::new(Rc::new(vec![2u8, 1u8, 5u8]), 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_f64();
        assert_eq!(result, 66.66f64);
        assert_eq!(instruction_execute_result.new_pc, 3usize);
    }
}