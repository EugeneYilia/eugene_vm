use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;

/// return
pub fn r#return(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    thread.pop_stack_frame();
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}


#[cfg(test)]
mod tests {
    use crate::core::bytecode_execution_engine::instruction::method::xreturn::r#return;
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_return() {
        let mut thread = Thread::new(None);
        thread.push_stack_frame(mock_stack_frame());
        thread.push_stack_frame(mock_stack_frame());
        assert_eq!(2usize, thread.get_stack_size());
        let instruction_execute_result = r#return(&mut CodeReader::new(vec![21u8, 32u8, 1u8, 2u8], 1usize), &mut thread);
        assert_eq!(1usize, thread.get_stack_size());
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }
}