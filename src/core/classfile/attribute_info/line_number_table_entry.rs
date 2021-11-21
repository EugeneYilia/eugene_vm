// 描述了java源码行号和字节码行号之间的对应关系，并不是运行时的必须属性，但默认会生成到Class文件之中
#[derive(Debug)]
pub struct LineNumberTableEntry {
    // 字节码的行号
    pub start_pc: u16,
    // java源码行号
    pub line_number: u16,
}