use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn get_static(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let static_field_index = code_reader.read_u16() as usize;
    // println!("{}", static_field_index);
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { class, .. } = stack_frame;
    let class = class.clone();
    let constant_info = class.constant_pool.get(static_field_index);
    match constant_info {
        ConstantInfo::MethodRef {
            class_index, name_and_type_index
        } => {
            println!("class_index: {}", class_index);
            println!("name_and_type_index: {}", name_and_type_index);
        }
        _ => {
            panic!("class: {:?}   static_field_index: {} should point to ConstantInfo::MethodRef", class, static_field_index);
        }
    }
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}