use crate::runtime::method_area::classfile::attribute_info::AttributeInfo;

#[derive(Debug)]
pub struct MemberInfo {
    pub access_flags : u16,
    pub name:String,
    pub name_index : u16,
    pub descriptor : String,
    pub descriptor_index:u16,
    pub attributes:Vec<AttributeInfo>
}