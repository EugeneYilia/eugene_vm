use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::thread::Thread;

pub fn iinc(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    let StackFrame {
        local_variable_table,
        ..
    } = stack_frame;
    let local_variable_index = code_reader.read_u8() as usize;
    let change_value = code_reader.read_u8() as i32;

    let local_variable_original_value = local_variable_table.get_variable_slot_mut(local_variable_index);
    match local_variable_original_value {
        VariableSlot::I32(value) => {
            *value += change_value;
        }
    }
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[cfg(test)]
mod tests {
    use crate::core::bytecode_execution_engine::instruction::math::inc::iinc;
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::stack::variable_slot::VariableSlot;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_iinc() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.local_variable_table.set_variable_slot(0usize, VariableSlot::I32(1i32));
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = iinc(&mut CodeReader::new(vec![0u8,0u8,3u8], 1usize), &mut thread);
        match thread.pop_stack_frame().local_variable_table.get_variable_slot_mut(0) {
            VariableSlot::I32(value) => {
                assert_eq!(*value, 4i32);
            }
        }
        assert_eq!(instruction_execute_result.new_pc, 3usize);
    }
}