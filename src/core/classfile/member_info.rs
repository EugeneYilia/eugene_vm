use crate::core::classfile::attribute_info::attribute_info::AttributeInfo;

// 因为field_info和method_info结构体中内容是一样的  因此采用member_info来表达
#[derive(Debug)]
pub struct MemberInfo {
    pub access_flags: u16,
    pub name: String,
    pub name_index: u16,
    pub descriptor: String,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl MemberInfo {
    // 返回方法特有的属性  并不是所有方法都有这个属性  比如说抽象方法、native方法、接口方法
    pub fn get_attribute_code(&self) -> Option<&AttributeInfo> {
        self.attributes.iter().find(|attribute_info|
            match attribute_info {
                AttributeInfo::Code { .. } => true,
                _ => false
            }
        )
    }

    // 返回字段特有的属性
    pub fn get_attribute_constant(&self) -> Option<&AttributeInfo> {
        self.attributes.iter().find(|attribute_info|
            match attribute_info {
                AttributeInfo::ConstantValue { .. } => true,
                _ => false
            }
        )
    }
}