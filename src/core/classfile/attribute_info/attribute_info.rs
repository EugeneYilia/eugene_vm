use crate::core::classfile::attribute_info::exception_table_entry::ExceptionTableEntry;
use crate::core::classfile::attribute_info::line_number_table_entry::LineNumberTableEntry;
use crate::core::classfile::attribute_info::local_variable_table_entry::LocalVariableTableEntry;

#[derive(Debug)]
pub enum AttributeInfo {
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTableEntry>,
        attributes: Vec<AttributeInfo>,
    },
    ConstantValue {
        constant_value_index: u16,
    },
    Deprecated,
    Exceptions {
        // 表示方法可能会抛出的受检异常，也就是方法描述时在throws关键字后面列举的异常
        // 其中的每一项是指向常量池中CONSTANT_Class_Info型常量的索引，代表了该受检异常的类型
        exception_index_table: Vec<u16>
    },
    EnclosingMethod,
    InnerClasses,
    LineNumberTable {
        line_number_table: Vec<LineNumberTableEntry>
    },
    LocalVariableTable {
        // 用于描述栈帧中局部变量表中的变量与java源码中定义的变量之间的关系，它也不是运行时必需的属性，但默认会生成到Class文件之中
        local_variable_table: Vec<LocalVariableTableEntry>
    },
    StackMapTable,
    Signature,
    SourceFile {
        source_file_index: u16,
    },
    SourceDebugExtension,
    Synthetic,
    LocalVariableTypeTable,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    AnnotationDefault,
    BootstrapMethods,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    MethodParameters,
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost,
    NestMembers,

    Unparsed {
        attribute_name: String,
        attribute_length: u32,
    },
}