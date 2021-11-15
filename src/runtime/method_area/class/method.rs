use crate::runtime::method_area::class::class_member::ClassMember;
use crate::core::classfile::member_info::MemberInfo;
use crate::core::classfile::attribute_info::attribute_info::AttributeInfo;

// access_flags   descriptor      name        descriptor
//  public          String       getName    (String name){}
#[derive(Debug)]
pub struct Method {
    pub class_member: ClassMember,
    pub max_stack: usize,
    pub max_locals: usize,
    pub code: Vec<u8>,
}

impl Method {
    pub fn new(member_info: MemberInfo) -> Method {
        let class_member = ClassMember::new(&member_info);
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
        }
    }
}