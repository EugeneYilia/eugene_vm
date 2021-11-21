use crate::constants::field_descriptor::{DOUBLE_FIELD_DESCRIPTOR, LONG_FIELD_DESCRIPTOR};
use crate::core::classfile::attribute_info::attribute_info::AttributeInfo;
use crate::core::classfile::member_info::MemberInfo;
use crate::runtime::method_area::class::class_member::ClassMember;

// access_flags   descriptor      name
//    public       String        author = "EugeneLiu"
#[derive(Debug)]
pub struct Field {
    class_member: ClassMember,
    // ConstantValue属性用于通知虚拟机在类或接口初始化阶段为被标志为ACC_STATIC的字段自动赋值，如接口中声明的字段，类中声明的静态常量字段。
    // 其它非ACC_STATIC的字段是在类的实例初始化方法中完成的。
    pub constant_value_index: Option<usize>,
}

impl Field {
    pub fn new(member_info: &MemberInfo) -> Field {
        let class_member = ClassMember::new(member_info);

        let constant_value_index = member_info.get_attribute_constant().map(|attribute_info| match attribute_info {
            AttributeInfo::ConstantValue {
                constant_value_index
            } => *constant_value_index as usize,
            _ => { panic!("Wow, that's amazing!") }
        });

        Field {
            class_member,
            constant_value_index,
        }
    }

    pub fn get_name(&self) -> &str {
        self.class_member.name.as_str()
    }

    pub fn get_descriptor(&self) -> &str {
        self.class_member.descriptor.as_str()
    }

    pub fn get_access_flags(&self) -> u16 {
        self.class_member.access_flags
    }

    pub fn is_need_two_slot(&self) -> bool {
        &self.class_member.descriptor == LONG_FIELD_DESCRIPTOR ||
            &self.class_member.descriptor == DOUBLE_FIELD_DESCRIPTOR
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