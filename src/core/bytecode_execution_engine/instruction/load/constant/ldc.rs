use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

/// 将int, float或String型常量值从常量池中推送至栈顶
pub fn ldc(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, class, .. } = stack_frame;
    let constant_pool_index = code_reader.read_u8();
    let constant_info = class.clone().constant_pool.get(constant_pool_index as usize);
    match constant_info {
        ConstantInfo::Integer(value) => operand_stack.push_i32(*value),
        ConstantInfo::Float(value) => operand_stack.push_f32(*value),
        // TODO: 使用更好的方式将String的ref推送到操作数栈上
        ConstantInfo::String(value) => operand_stack.push_i32(*value as i32),
        _ => panic!("Class Format Error: {:?}", class.clone())
    }

    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

/// 将int, float或String型常量值从常量池中推送至栈顶(宽索引)
pub fn ldc_w(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, class, .. } = stack_frame;
    // 宽索引 u16
    let constant_pool_index = code_reader.read_u16();
    let constant_info = class.clone().constant_pool.get(constant_pool_index as usize);
    match constant_info {
        ConstantInfo::Integer(value) => operand_stack.push_i32(*value),
        ConstantInfo::Float(value) => operand_stack.push_f32(*value),
        // TODO: 使用更好的方式将String的ref推送到操作数栈上
        ConstantInfo::String(value) => operand_stack.push_i32(*value as i32),
        _ => panic!("Class Format Error: {:?}", class.clone())
    }
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

/// 将long或double型常量值从常量池中推送至栈顶(宽索引)
pub fn ldc2_w(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, class, .. } = stack_frame;
    // 宽索引 u16
    let constant_pool_index = code_reader.read_u16();
    let constant_info = class.clone().constant_pool.get(constant_pool_index as usize);
    match constant_info {
        ConstantInfo::Long(value) => operand_stack.push_i64(*value),
        ConstantInfo::Double(value) => operand_stack.push_f64(*value),
        _ => panic!("Class Format Error: {:?}", class.clone())
    }
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[cfg(test)]
mod tests {
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_ldc_i32() {
        let stack_frame = mock_stack_frame();
        let mut thread = Thread::new(None);

        let result = thread.pop_stack_frame().operand_stack.pop_f32();
    }

    #[test]
    fn test_ldc_f32(){
        let stack_frame = mock_stack_frame();
        let mut thread = Thread::new(None);

    }

    #[test]
    fn test_ldc_string(){
        let stack_frame = mock_stack_frame();
        let mut thread = Thread::new(None);

    }

    #[test]
    fn test_ldc_w_i32() {
        let stack_frame = mock_stack_frame();
        let mut thread = Thread::new(None);

    }

    #[test]
    fn test_ldc_w_f32(){
        let stack_frame = mock_stack_frame();
        let mut thread = Thread::new(None);

    }

    #[test]
    fn test_ldc_w_string(){
        let stack_frame = mock_stack_frame();
        let mut thread =Thread::new(None);

    }

    #[test]
    fn test_ldc2_w_long() {
        let stack_frame = mock_stack_frame();
        let mut thread = Thread::new(None);

    }

    #[test]
    fn test_ldc2_w_double(){
        let stack_frame = mock_stack_frame();
        let mut thread = Thread::new(None);


    }
}