use crate::constants::access_flags::{ACCESS_PUBLIC, ACCESS_STATIC};
use crate::core::classfile::attribute_info::attribute_info::AttributeInfo;
use crate::core::classfile::member_info::MemberInfo;
use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;

#[derive(Debug)]
pub struct ClassFile {
    pub major_version: u16,
    pub minor_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<MemberInfo>,
    pub methods: Vec<MemberInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn main_method(&self) -> &MemberInfo {
        self.methods
            .iter()
            .find(|member_info| {
                member_info.name == "main"
                    && member_info.descriptor == "([Ljava/lang/String;)V"
                    && member_info.access_flags & ACCESS_STATIC != 0
                    && member_info.access_flags & ACCESS_PUBLIC != 0
            })
            .expect("Main method not found")
    }

    pub fn get_class_name(&self) -> &str {
        self.constant_pool.get_class_name(self.this_class as usize)
    }

    pub fn get_super_class_name(&self) -> &str {
        if self.super_class > 0 {
            self.constant_pool.get_class_name(self.super_class as usize)
        } else {
            ""
        }
    }
}