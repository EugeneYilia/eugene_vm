use byteorder::{BigEndian, ByteOrder};
use crate::runtime::method_area::classfile::version_info::VersionInfo;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::method_area::classfile::member_info::MemberInfo;
use crate::runtime::method_area::classfile::attribute_info::exception_table_entry::ExceptionTableEntry;
use crate::runtime::method_area::classfile::attribute_info::line_number_table_entry::LineNumberTableEntry;
use crate::runtime::method_area::classfile::attribute_info::local_variable_table_entry::LocalVariableTableEntry;
use crate::runtime::method_area::classfile::attribute_info::attribute_info::AttributeInfo;
use crate::runtime::method_area::classfile::classfile::ClassFile;
use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;

use crate::core::annotation::loop_n;

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
    fn read_and_check_version(&self) -> (VersionInfo, &[u8]);
    fn read_constant_info(&self) -> (ConstantInfo, &[u8]);
    fn read_constant_pool(&self) -> (ConstantPool, &[u8]);
    fn read_access_flags(&self) -> (u16, &[u8]);
    fn read_this_class(&self) -> (u16, &[u8]);
    fn read_super_class(&self) -> (u16, &[u8]);
    fn read_interfaces(&self) -> (Vec<u16>, &[u8]);
    fn read_member(&self, constant_pool: &ConstantPool) -> (MemberInfo, &[u8]);
    fn read_members(&self, constant_pool: &ConstantPool) -> (Vec<MemberInfo>, &[u8]);
    fn read_exception_table(&self) -> (Vec<ExceptionTableEntry>, &[u8]);
    fn read_line_number_table(&self) -> (Vec<LineNumberTableEntry>, &[u8]);
    fn read_local_variable_table(&self) -> (Vec<LocalVariableTableEntry>, &[u8]);
    fn read_attribute(&self, constant_pool: &ConstantPool) -> (AttributeInfo, &[u8]);
    fn find_attributes(&self, constant_pool: &ConstantPool) -> (Vec<AttributeInfo>, &[u8]);
    fn parse(&self) -> ClassFile;
}

impl ClassReader for [u8] {
    fn read_u8(&self) -> (u8, &[u8]) {
        let (temp, left) = self.split_at(1);
        (temp[0], left)
    }

    fn read_u16(&self) -> (u16, &[u8]) {
        let (temp, left) = self.split_at(2);
        (BigEndian::read_u16(temp), left)
    }

    fn read_u16s(&self) -> (Vec<u16>, &[u8]) {
        let (u16_length, mut left) = self.read_u16();
        let mut target: Vec<u16> = Vec::with_capacity(u16_length as usize);
        loopn!(u16_length,{
            let (u16_value, u8_left) = left.read_u16();
            left = u8_left;
            target.push(u16_value);
        });
        (target, left)
    }

    fn read_u32(&self) -> (u32, &[u8]) {
        todo!()
    }

    fn read_i32(&self) -> (i32, &[u8]) {
        todo!()
    }

    fn read_i64(&self) -> (i64, &[u8]) {
        todo!()
    }

    fn read_f32(&self) -> (f32, &[u8]) {
        todo!()
    }

    fn read_f64(&self) -> (f64, &[u8]) {
        todo!()
    }

    fn read_bytes(&self, n: usize) -> (&[u8], &[u8]) {
        todo!()
    }

    fn read_and_check_magic_number(&self) -> (u32, &[u8]) {
        todo!()
    }

    fn read_and_check_version(&self) -> (VersionInfo, &[u8]) {
        todo!()
    }

    fn read_constant_info(&self) -> (ConstantInfo, &[u8]) {
        todo!()
    }

    fn read_constant_pool(&self) -> (ConstantPool, &[u8]) {
        todo!()
    }

    fn read_access_flags(&self) -> (u16, &[u8]) {
        todo!()
    }

    fn read_this_class(&self) -> (u16, &[u8]) {
        todo!()
    }

    fn read_super_class(&self) -> (u16, &[u8]) {
        todo!()
    }

    fn read_interfaces(&self) -> (Vec<u16>, &[u8]) {
        todo!()
    }

    fn read_member(&self, constant_pool: &ConstantPool) -> (MemberInfo, &[u8]) {
        todo!()
    }

    fn read_members(&self, constant_pool: &ConstantPool) -> (Vec<MemberInfo>, &[u8]) {
        todo!()
    }

    fn read_exception_table(&self) -> (Vec<ExceptionTableEntry>, &[u8]) {
        todo!()
    }

    fn read_line_number_table(&self) -> (Vec<LineNumberTableEntry>, &[u8]) {
        todo!()
    }

    fn read_local_variable_table(&self) -> (Vec<LocalVariableTableEntry>, &[u8]) {
        todo!()
    }

    fn read_attribute(&self, constant_pool: &ConstantPool) -> (AttributeInfo, &[u8]) {
        todo!()
    }

    fn find_attributes(&self, constant_pool: &ConstantPool) -> (Vec<AttributeInfo>, &[u8]) {
        todo!()
    }

    fn parse(&self) -> ClassFile {
        todo!()
    }
}

//
#[test]
pub fn test_read_u16s(){
    let u8_array :[u8;10] = [0,2,1,1,0,7,0,2,1,2];
    // result: 257  7   left 0 2 1 2
    let (result,left) = u8_array.read_u16s();
    println!("{:?}",result);
    println!();
    println!("{:?}",left);
}