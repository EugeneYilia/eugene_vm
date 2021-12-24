use crate::core::classfile::member_info::MemberInfo;

#[derive(Debug)]
pub struct ClassMember {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
}

impl ClassMember {
    pub fn new(member_info: &MemberInfo) -> ClassMember {
        ClassMember {
            access_flags: member_info.access_flags.clone(),
            name: member_info.name.clone(),
            descriptor: member_info.descriptor.clone(),
        }
    }
}