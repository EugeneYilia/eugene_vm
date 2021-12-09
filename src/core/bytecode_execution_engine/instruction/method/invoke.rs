use std::rc::Rc;
use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

/// 调用实例方法
pub fn invoke_virtual(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack,class, .. } = stack_frame;
    let class = Rc::clone(class);
    let virtual_method_index = code_reader.read_u16() as usize;
    if let ConstantInfo::MethodRef {class_index,name_and_type_index} = class.constant_pool.get(virtual_method_index) {

    } else {

    }

    let value = operand_stack.pop_i32();
    println!("{}", value);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

