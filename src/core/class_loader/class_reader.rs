use crate::runtime::method_area::classfile::version_info::VersionInfo;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::method_area::constant_pool::ConstantPool;
use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;
use crate::runtime::method_area::classfile::member_info::MemberInfo;

trait ClassReader {
    fn read_u8(&self) -> (u8, &[u8]);
    fn read_u16(&self) -> (u16, &[u8]);
    fn read_u16s(&self) -> (Vec<u16>, &[u8]);
    fn read_u32(&self) -> (u32, &[u8]);

    fn read_i32(&self) -> (i32, &[u8]);
    fn read_i64(&self) -> (i64, &[u8]);

    fn read_f32(&self) -> (f32, &[u8]);
    fn read_f64(&self) -> (f64, &[u8]);

    fn read_bytes(&self, n: usize) -> (&[u8], &[u8]);

    fn read_and_check_magic_number(&self) -> (u32, &[u8]);
    fn read_and_check_version(&self)->(VersionInfo,&[u8]);
    fn read_constant_info(&self)->(ConstantInfo,&[u8]);
    fn read_constant_pool(&self)->(ConstantPool,&[u8]);
    fn read_access_flags(&self)->(u16,&[u8]);
    fn read_this_class(&self)->(u16,&[u8]);
    fn read_super_class(&self)->(u16,&[u8]);
    fn read_interfaces(&self)->(Vec<u16>,&[u8]);
    fn read_member(&self, constant_pool:&ConstantPool) -> (MemberInfo,&[u8]);
    fn read_members(&self, constant_pool:&ConstantPool) -> (Vec<MemberInfo>,&[u8]);


}