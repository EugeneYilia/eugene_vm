
// 栈帧 与 源码局部变量遍之间的关联
#[derive(Debug)]
pub struct LocalVariableTableEntry{
    // 局部变量的生命周期开始的字节码偏移
    pub start_pc : u16,
    // 局部变量作用范围覆盖的长度
    pub length : u16,
    // 指向常量池中CONSTANT_Utf8_info的索引，分别代表了局部变量的名称和描述符
    pub name_index : u16,
    pub descriptor_index : u16,
    // 表示这个局部变量在栈帧的局部变量表中的变量槽的位置
    pub index : u16
}