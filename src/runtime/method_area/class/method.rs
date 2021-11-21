use crate::core::classfile::attribute_info::attribute_info::AttributeInfo;
use crate::core::classfile::member_info::MemberInfo;
use crate::runtime::method_area::class::class_member::ClassMember;

// access_flags   descriptor      name        descriptor
//  public          String       getName    (String name){}
#[derive(Debug)]
pub struct Method {
    class_member: ClassMember,
    pub max_stack: usize,
    pub max_locals: usize,
    pub code: Vec<u8>,
}

impl Method {
    pub fn new(member_info: &MemberInfo) -> Method {
        let class_member = ClassMember::new(member_info);
        let attribute_info = member_info.get_attribute_code();
        match attribute_info {
            Some(AttributeInfo::Code {
                     max_stack,
                     max_locals,
                     code,
                     ..
                 }) => {
                Method {
                    class_member,
                    max_stack: *max_stack as usize,
                    max_locals: *max_locals as usize,
                    code: code.clone(),
                }
            }
            None => {
                // 抽象方法  接口方法  native方法
                Method {
                    class_member,
                    max_stack: 0usize,
                    max_locals: 0usize,
                    code: Vec::new(),
                }
            }
            _ => {
                panic!("Wow, that's amazing!")
            }
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
}