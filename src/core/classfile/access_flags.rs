// 定义来源于jdk
// Access flags values, defined in
// - https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-4.html#jvms-4.1-200-E.1
// - https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-4.html#jvms-4.5-200-A.1
// - https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-4.html#jvms-4.6-200-A.1
// - https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-4.html#jvms-4.7.25

// int ACC_PUBLIC = 0x0001; // class, field, method
// int ACC_PRIVATE = 0x0002; // class, field, method
// int ACC_PROTECTED = 0x0004; // class, field, method
// int ACC_STATIC = 0x0008; // field, method
// int ACC_FINAL = 0x0010; // class, field, method, parameter
// int ACC_SUPER = 0x0020; // class
// int ACC_SYNCHRONIZED = 0x0020; // method
// int ACC_OPEN = 0x0020; // module
// int ACC_TRANSITIVE = 0x0020; // module requires
// int ACC_VOLATILE = 0x0040; // field
// int ACC_BRIDGE = 0x0040; // method
// int ACC_STATIC_PHASE = 0x0040; // module requires
// int ACC_VARARGS = 0x0080; // method
// int ACC_TRANSIENT = 0x0080; // field
// int ACC_NATIVE = 0x0100; // method
// int ACC_INTERFACE = 0x0200; // class
// int ACC_ABSTRACT = 0x0400; // class, method
// int ACC_STRICT = 0x0800; // method
// int ACC_SYNTHETIC = 0x1000; // class, field, method, parameter, module *
// int ACC_ANNOTATION = 0x2000; // class
// int ACC_ENUM = 0x4000; // class(?) field inner
// int ACC_MANDATED = 0x8000; // field, method, parameter, module, module *
// int ACC_MODULE = 0x8000; // class

pub const ACCESS_PUBLIC: u16 = 0x0001; // class, field, method
pub const ACCESS_PRIVATE: u16 = 0x0002; // class, field, method
pub const ACCESS_PROTECTED: u16 = 0x0004; // class, field, method
pub const ACCESS_STATIC: u16 = 0x0008; // field, method
// 如果给方法的parameter加了final关键字的话  在方法体中不可以再对参数进行修改 否则会报错
// 在kotlin中 方法参数默认就是final的不可以对方法参数进行修改  java中默认方法参数是非final的可以对方法参数进行修改
pub const ACCESS_FINAL: u16 = 0x0010; // class, field, method, parameter
pub const ACCESS_SUPER: u16 = 0x0020; // class
pub const ACCESS_SYNCHRONIZED: u16 = 0x0020; // method
pub const ACCESS_OPEN: u16 = 0x0020; // module
pub const ACCESS_TRANSITIVE: u16 = 0x0020; // module requires
pub const ACCESS_VOLATILE: u16 = 0x0040; // field
pub const ACCESS_BRIDGE: u16 = 0x0040;// method   编译器生成的方法
pub const ACCESS_STATIC_PHASE: u16 = 0x0040;// module requires
pub const ACCESS_VARARGS: u16 = 0x0080;// method
pub const ACCESS_TRANSIENT: u16 = 0x0080;// field
pub const ACCESS_NATIVE: u16 = 0x0100;// method
pub const ACCESS_INTERFACE: u16 = 0x0200;// class
pub const ACCESS_ABSTRACT: u16 = 0x0400;// class, method
// 在java中，浮点表示和计算与平台有关
// strictfp修饰符可确保跨不同jvm和平台的所有浮点运算将提供IEEE 754预测的一致且相同的结果
// 当我们使用strictfp时，jvm使用可以由标准java float或double表示的值执行浮点计算，从而确保计算结果将在所有jvm和平台之间完全匹配
// 该修饰符在所有平台和处理器体系结构中将产生完全相同的结果  比如Double.MAX_VALUE默认在每个平台中都有不同的表示形式
// 上述情况如果使用了strictfp  我们就可以确保给定的计算将始终得出相同的值
pub const ACCESS_STRICT: u16 = 0x0800;// method
pub const ACCESS_SYNTHETIC: u16 = 0x1000;// class, field, method, parameter, module *
pub const ACCESS_ANNOTATION: u16 = 0x2000;// class
pub const ACCESS_ENUM: u16 = 0x4000;// class(?) field inner
pub const ACCESS_MANDATED: u16 = 0x8000;// field, method, parameter, module, module *
pub const ACCESS_MODULE: u16 = 0x8000;// class

// 基本不考虑
pub const ACCESS_RECORD: u32 = 0x10000;// class
pub const ACCESS_DEPRECATED: u32 = 0x20000;// class, field, method