// 异常表记录者字节码从[start,end)出现的异常类型为catch_type跳转到handle_pc去处理   catch_type为0时，任意异常都需要跳转到handler_pc位置去处理
#[derive(Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handle_pc: u16,
    pub catch_type: u16,
}