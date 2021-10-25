use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;

#[derive(Debug)]
pub struct ClassFile {
    pub major_version: u16,
    pub minor_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags:u16,
    pub this_class:u16,
    pub super_class:u16,
    pub interfaces:Vec<u16>,

}

impl ClassFile {
    fn x(){
        let a = b"123";
    }
}