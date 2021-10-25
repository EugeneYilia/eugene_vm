#[derive(Debug)]
pub enum AttributeInfo {
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<>,
        attributes: Vec<AttributeInfo>
    },
    ConstantValue {
        constant_value_index : u16,
    },
    Deprecated,
    Exceptions,
    EnclosingMethod,
    InnerClasses,
    LineNumberTable,
    LocalVariableTable,
    StackMapTable,
    Signature,
    SourceFile {
        source_file_index : u16,
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
        attribute_name : String,
        attribute_length: u32,
    }
}