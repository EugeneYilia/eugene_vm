use crate::core::constant::constant_pool::ConstantPool;

#[derive(Debug)]
pub struct Class {
    pub access_flags : u16,




    pub constant_pool: ConstantPool,

    pub name:String,
}