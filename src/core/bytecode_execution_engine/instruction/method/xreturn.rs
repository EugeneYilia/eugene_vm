use std::cell::RefMut;

use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;

/// return
pub fn r#return(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    thread.pop_stack_frame();
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    use crate::core::bytecode_execution_engine::instruction::method::xreturn::r#return;
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_return() {
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(mock_stack_frame());
        thread.deref().borrow_mut().push_stack_frame(mock_stack_frame());
        assert_eq!(2usize, thread.deref().borrow_mut().get_stack_size());
        let instruction_execute_result = r#return(&mut CodeReader::new(vec![21u8, 32u8, 1u8, 2u8], 1usize), thread.deref().borrow_mut());
        assert_eq!(1usize, thread.deref().borrow_mut().get_stack_size());
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }
}