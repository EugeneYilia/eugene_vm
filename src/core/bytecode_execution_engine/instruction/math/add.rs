use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn iadd(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i32();
    let second = operand_stack.pop_i32();
    let result = first + second;
    operand_stack.push_i32(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn ladd(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i64();
    let second = operand_stack.pop_i64();
    let result = first + second;
    operand_stack.push_i64(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn fadd(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_f32();
    let second = operand_stack.pop_f32();
    let result = first + second;
    operand_stack.push_f32(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn dadd(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_f64();
    let second = operand_stack.pop_f64();
    let result = first + second;
    operand_stack.push_f64(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::rc::Rc;

    use crate::core::bytecode_execution_engine::instruction::math::add::{dadd, fadd, iadd, ladd};
    use crate::core::classfile::member_info::MemberInfo;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::method_area::class::class::Class;
    use crate::runtime::method_area::class::method::Method;
    use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;
    use crate::runtime::stack::stack_frame::StackFrame;
    use crate::runtime::stack::variables_table::VariableTable;
    use crate::runtime::thread::Thread;

    fn mock_method() -> Rc<Method> {
        Rc::new(Method::new(&MemberInfo {
            access_flags: 0u16,
            name: "".to_string(),
            name_index: 0u16,
            descriptor_index: 0u16,
            descriptor: "".to_string(),
            attributes: Vec::new(),
        }))
    }

    fn mock_class() -> Rc<Class> {
        Rc::new(Class {
            access_flags: 0u16,
            class_name: "".to_string(),
            constant_pool: ConstantPool {
                constant_info_map: BTreeMap::new()
            },
            fields: Vec::new(),
            methods: Vec::new(),
            super_class: None,
            next_instance_slot_id: 0usize,
            next_static_slot_id: 0usize,
            static_variable_table: VariableTable::new(),
        })
    }

    fn mock_stack_frame() -> StackFrame {
        StackFrame::new(mock_class(), mock_method())
    }

    #[test]
    fn test_iadd() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_i32(2i32);
        stack_frame.operand_stack.push_i32(4i32);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = iadd(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result, 6i32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_ladd() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_i64(12345678969i64);
        stack_frame.operand_stack.push_i64(2997924580i64);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = ladd(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i64();
        assert_eq!(result, 15343603549i64);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_fadd() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f32(3.1415926f32);
        stack_frame.operand_stack.push_f32(3.1415926f32);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = fadd(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_f32();
        assert_eq!(result, 6.2831852f32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_dadd() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f64(2.71828182845f64);
        stack_frame.operand_stack.push_f64(3.1415926535897926f64);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = dadd(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_f64();
        assert_eq!(result, 5.8598744820397926f64);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }
}