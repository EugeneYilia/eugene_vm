use std::rc::Rc;

use crate::constants::descriptor::{DOUBLE_DESCRIPTOR, LONG_DESCRIPTOR};
use crate::core::classfile::attribute_info::attribute_info::AttributeInfo;
use crate::core::classfile::member_info::MemberInfo;
use crate::runtime::method_area::class::class_member::ClassMember;

// access_flags   descriptor      name
//    public       String        author = "EugeneLiu"
#[derive(Debug)]
pub struct Field {
    class_member: Rc<ClassMember>,
    // ConstantValue属性用于通知虚拟机在类或接口初始化阶段为被标志为ACC_STATIC的字段自动赋值，如接口中声明的字段，类中声明的静态常量字段。
    // 其它非ACC_STATIC的字段是在类的实例初始化方法中完成的。
    pub constant_value_index: Option<usize>,
}

impl Field {
    pub fn new(member_info: &MemberInfo) -> Field {
        let class_member = Rc::new(ClassMember::new(member_info));

        let constant_value_index = member_info.get_attribute_constant().map(|attribute_info| match attribute_info {
            AttributeInfo::ConstantValue {
                constant_value_index
            } => *constant_value_index as usize,
            _ => {
                unreachable!();
            }
        });

        Field {
            class_member,
            constant_value_index,
        }
    }

    pub fn get_name(&self) -> &str {
        self.class_member.name.as_str()
    }

    pub fn get_descriptor(&self) -> char {
        self.class_member.descriptor.chars().next().unwrap()
    }

    pub fn get_access_flags(&self) -> u16 {
        self.class_member.access_flags
    }

    pub fn get_class_member(&self) -> Rc<ClassMember> {
        Rc::clone(&self.class_member)
    }

    pub fn is_need_two_slot(&self) -> bool {
        self.get_descriptor() == LONG_DESCRIPTOR ||
            self.get_descriptor() == DOUBLE_DESCRIPTOR
    }
}

#[test]
fn test_field_descriptor() {
    let field = Field::new(&MemberInfo {
        access_flags: 0u16,
        name: "".to_string(),
        name_index: 0u16,
        descriptor_index: 0u16,
        descriptor: "D".to_string(),
        attributes: Vec::new(),
    });
    println!("{}", field.is_need_two_slot())
}

#[test]
fn test_get_field_descriptor() {
    let descriptor = "Ljava/lang/String;".to_owned();
    let _real_descriptor = descriptor.chars().next().unwrap();
    let _real_descriptor = descriptor.chars().next().unwrap();
    let real_descriptor = descriptor.chars().next().unwrap();
    println!("{}", real_descriptor);
    println!("{}", real_descriptor);
    println!("{}", descriptor);
    println!("{}", descriptor);
}