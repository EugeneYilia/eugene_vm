use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;

#[derive(Debug)]
pub struct Class {
    pub access_flags:u16,
    pub constant_pool:ConstantPool,
    pub class_name: String,

}