#[derive(Debug)]
pub enum ConstantInfo {
    Integer(i32),
    Long(i64),

    Float(f32),
    Double(f64),

    UTF8(String),
    String(u16),

    // 描述类的常量信息
    Class {
        // class的名称的引用
        name_index: u16
    },

    // 描述一个方法或者成员变量
    NameAndType {
        //方法或变量名称的引用
        name_index: u16,
        // 方法的描述符的引用
        descriptor_index: u16,
    },

    // 描述一个字段的引用
    FieldRef {
        // 指向该字段所在的Class的引用
        class_index: u16,
        // 指向描述该字段名称和描述符的引用
        name_and_type_index: u16,
    },

    // 描述一个方法的引用
    MethodRef {
        // 指向该方法所在的Class的引用
        class_index: u16,
        // 指向描述该方法名称和描述符的引用
        name_and_type_index: u16,
    },

    // 描述一个隶属于接口的方法的引用
    InterfaceMethodRef{

    }
}