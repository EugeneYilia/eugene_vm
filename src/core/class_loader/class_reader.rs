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
use crate::runtime::method_area::constant_pool::constant_info_tag::*;

use crate::core::r#macro::loop_n;
use std::collections::HashMap;

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
    fn read_attributes(&self, constant_pool: &ConstantPool) -> (Vec<AttributeInfo>, &[u8]);
    fn parse(&self) -> ClassFile;
}

//
// ClassFile {
//     u4             magic;
//     u2             minor_version;
//     u2             major_version;
//     u2             constant_pool_count;
//     cp_info        constant_pool[constant_pool_count-1];
//     u2             access_flags;
//     u2             this_class;
//     u2             super_class;
//     u2             interfaces_count;
//     u2             interfaces[interfaces_count];
//     u2             fields_count;
//     field_info     fields[fields_count];
//     u2             methods_count;
//     method_info    methods[methods_count];
//     u2             attributes_count;
//     attribute_info attributes[attributes_count];
// }
// Class文件结构：https://blog.csdn.net/qq_39888626/article/details/120606371
//
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
        let (u16_amount, mut left) = self.read_u16();
        let mut target: Vec<u16> = Vec::with_capacity(u16_amount as usize);
        loopn!(u16_amount,{
            let (u16_value, u8_left) = left.read_u16();
            left = u8_left;
            target.push(u16_value);
        });
        (target, left)
    }

    fn read_u32(&self) -> (u32, &[u8]) {
        let (temp, left) = self.split_at(4);
        (BigEndian::read_u32(temp), left)
    }

    fn read_i32(&self) -> (i32, &[u8]) {
        let (temp, left) = self.split_at(4);
        (BigEndian::read_i32(temp), left)
    }

    fn read_i64(&self) -> (i64, &[u8]) {
        let (temp, left) = self.split_at(8);
        (BigEndian::read_i64(temp), left)
    }

    fn read_f32(&self) -> (f32, &[u8]) {
        let (temp, left) = self.split_at(4);
        (BigEndian::read_f32(temp), left)
    }

    fn read_f64(&self) -> (f64, &[u8]) {
        let (temp, left) = self.split_at(8);
        (BigEndian::read_f64(temp), left)
    }

    fn read_bytes(&self, n: usize) -> (&[u8], &[u8]) {
        self.split_at(n)
    }

    // Magic Number:   0xCAFEBABE    u32
    fn read_and_check_magic_number(&self) -> (u32, &[u8]) {
        let result = self.read_u32();
        let (magic_number, _left) = result;
        assert_eq!(magic_number, 0xCAFEBABE);
        result
    }
    //                                          major_version   minor_version
    // 目前校验的版本是按照jdk1.8编译出来的class文件       52              0
    fn read_and_check_version(&self) -> (VersionInfo, &[u8]) {
        let (minor_version, left) = self.read_u16();
        let (major_version, left) = left.read_u16();
        assert_eq!(major_version, 52);
        assert_eq!(minor_version, 0);
        let version_info = VersionInfo {
            major_version,
            minor_version,
        };
        (version_info, left)
    }

    fn read_constant_info(&self) -> (ConstantInfo, &[u8]) {
        let (constant_info_tag, left) = self.read_u8();
        match constant_info_tag {
            CONSTANT_UTF8_INFO_TAG => {
                let (utf8_string_length, left) = left.read_u16();
                let (bytes, left) = left.read_bytes(utf8_string_length as usize);
                // from_java_cesu8
                // Convert Java's modified UTF-8 data to a Rust string, re-encoding only if necessary.
                // Returns an error if the data cannot be represented as valid UTF-8.
                let modified_utf8_result = cesu8::from_java_cesu8(bytes);
                let value: String = match modified_utf8_result {
                    Ok(modified_utf8_str) => modified_utf8_str.to_string(),
                    Err(error) => panic!("constant_utf8_info {:?} is invalid Modified UTF-8 sequence: {}", bytes, error),
                };

                (ConstantInfo::UTF8(value), left)
            }
            CONSTANT_INTEGER_INFO_TAG => {
                let (value, left) = left.read_i32();
                (ConstantInfo::Integer(value), left)
            }
            CONSTANT_FLOAT_INFO_TAG => {
                let (value, left) = left.read_f32();
                (ConstantInfo::Float(value), left)
            }
            CONSTANT_LONG_INFO_TAG => {
                let (value, left) = left.read_i64();
                (ConstantInfo::Long(value), left)
            }
            CONSTANT_DOUBLE_INFO_TAG => {
                let (value, left) = left.read_f64();
                (ConstantInfo::Double(value), left)
            }
            CONSTANT_CLASS_INFO_TAG => {
                let (name_index, left) = left.read_u16();
                (ConstantInfo::Class { name_index }, left)
            }
            CONSTANT_STRING_INFO_TAG => {
                let (value, left) = left.read_u16();
                (ConstantInfo::String(value), left)
            }
            CONSTANT_FIELD_REF_INFO_TAG => {
                let (class_index, left) = left.read_u16();
                let (name_and_type_index, left) = left.read_u16();
                (ConstantInfo::FieldRef { class_index, name_and_type_index }, left)
            }
            CONSTANT_METHOD_REF_INFO_TAG => {
                let (class_index, left) = left.read_u16();
                let (name_and_type_index, left) = left.read_u16();
                (ConstantInfo::MethodRef { class_index, name_and_type_index }, left)
            }
            CONSTANT_INTERFACE_METHOD_REF_INFO_TAG => {
                let (class_index, left) = left.read_u16();
                let (name_and_type_index, left) = left.read_u16();
                (ConstantInfo::InterfaceMethodRef { class_index, name_and_type_index }, left)
            }
            CONSTANT_NAME_AND_TYPE_INFO_TAG => {
                let (name_index, left) = left.read_u16();
                let (descriptor_index, left) = left.read_u16();
                (ConstantInfo::NameAndType { name_index, descriptor_index }, left)
            }
            _ => {
                panic!("Wrong constant_info_tag type");
            }
        }
    }

    fn read_constant_pool(&self) -> (ConstantPool, &[u8]) {
        let (constant_pool_count, left) = self.read_u16();
        // let mut constant_pool = ConstantPool{constant_info_map:HashMap::with_capacity()}
        todo!()
    }

    fn read_access_flags(&self) -> (u16, &[u8]) {
        self.read_u16()
    }

    fn read_this_class(&self) -> (u16, &[u8]) {
        self.read_u16()
    }

    fn read_super_class(&self) -> (u16, &[u8]) {
        self.read_u16()
    }

    fn read_interfaces(&self) -> (Vec<u16>, &[u8]) {
        self.read_u16s()
    }

    fn read_member(&self, constant_pool: &ConstantPool) -> (MemberInfo, &[u8]) {
        let (access_flags, left) = self.read_access_flags();
        let (name_index, left) = left.read_u16();
        let (descriptor_index, left) = left.read_u16();

        let (attributes, left) = left.read_attributes(constant_pool);
        let name = match constant_pool.get(name_index as usize) {
            ConstantInfo::UTF8(ref name) => name.to_owned(),
            _ => panic!("name_index doesn't point to UTF8 String")
        };
        let descriptor = match constant_pool.get(descriptor_index as usize) {
            ConstantInfo::UTF8(ref name) => name.to_owned(),
            _ => panic!("descriptor_index doesn't point to UTF8 String")
        };
        (
            MemberInfo {
                access_flags,
                name,
                name_index,
                descriptor,
                descriptor_index,
                attributes,
            },
            left
        )
    }

    fn read_members(&self, constant_pool: &ConstantPool) -> (Vec<MemberInfo>, &[u8]) {
        let (member_info_amount, left) = self.read_u16();
        let mut member_info_vec = Vec::with_capacity(member_info_amount as usize);
        loopn!(member_info_amount,{
            let (member_info, left) = left.read_member(constant_pool);
            member_info_vec.push(member_info);
        });
        (member_info_vec, left)
    }

    fn read_exception_table(&self) -> (Vec<ExceptionTableEntry>, &[u8]) {
        let (exception_table_length, left) = self.read_u16();
        let mut exception_table: Vec<ExceptionTableEntry> = Vec::with_capacity(exception_table_length as usize);
        loopn!(exception_table_length, {
            let (start_pc, left) = left.read_u16();
            let (end_pc, left) = left.read_u16();
            let (handle_pc, left) = left.read_u16();
            let (catch_type, left) = left.read_u16();
            let exception_table_entry = ExceptionTableEntry{ start_pc, end_pc, handle_pc, catch_type };
            exception_table.push(exception_table_entry);
        });
        (exception_table, left)
    }

    fn read_line_number_table(&self) -> (Vec<LineNumberTableEntry>, &[u8]) {
        let (line_number_table_length, left) = self.read_u16();
        let mut line_number_table: Vec<LineNumberTableEntry> = Vec::with_capacity(line_number_table_length as usize);
        loopn!(line_number_table_length, {
            let start_pc = left.read_u16();
            let line_number = left.read_u16();
            let line_number_table_entry = LineNumberTableEntry{ start_pc, line_number };
            line_number_table.push(line_number_table_entry)
        });
        (line_number_table, left)
    }

    fn read_local_variable_table(&self) -> (Vec<LocalVariableTableEntry>, &[u8]) {
        let (local_variable_table_length, left) = self.read_u16();
        let mut local_variable_table = Vec::with_capacity(local_variable_table_length as usize);
        loopn!(local_variable_table_length, {
            let (start_pc, left) = left.read_u16();
            let (length, left) = left.read_u16();
            let (name_index, left) = left.read_u16();
            let (descriptor_index, left) = left.read_u16();
            let (index, left) = left.read_u16();
            let local_variable_table_entry = LocalVariableTableEntry{ start_pc, length, name_index, descriptor_index, index};
            local_variable_table.push(local_variable_table_entry);
        });
        (local_variable_table, left)
    }

    fn read_attribute(&self, constant_pool: &ConstantPool) -> (AttributeInfo, &[u8]) {
        let (attribute_name_index, left) = self.read_u16();
        let attribute_name = match constant_pool.get(attribute_name_index as usize) {
            ConstantInfo::UTF8(attribute_name) => attribute_name,
            _ => panic!("attribute_name_index doesn't point to UTF8 String")
        };
        let (attribute_length, left) = left.read_u32();

        match attribute_name.as_str() {
            "Code" => {
                let (max_stack, left) = left.read_u16();
                let (max_locals, left) = left.read_u16();
                let (code_length, left) = left.read_u32();
                let (code, left) = left.read_bytes(code_length as usize);
                let (exception_table, left) = left.read_exception_table();
                let (attributes, left) = left.read_attributes(constant_pool);

                (
                    AttributeInfo::Code {
                        max_stack,
                        max_locals,
                        code: code.to_vec(),
                        exception_table,
                        attributes,
                    },
                    left
                )
            }
            "ConstantValue" => {
                let (constant_value_index, left) = left.read_u16();
                (
                    AttributeInfo::ConstantValue {
                        constant_value_index
                    },
                    left
                )
            }
            "Deprecated" => {
                (
                    AttributeInfo::Deprecated,
                    left
                )
            }
            "Exceptions" => {
                let (exception_index_table, left) = left.read_u16s();
                (
                    AttributeInfo::Exceptions {
                        exception_index_table
                    },
                    left
                )
            }
            "EnclosingMethod" => {
                (
                    AttributeInfo::EnclosingMethod,
                    left
                )
            }
            "InnerClasses" => {
                (
                    AttributeInfo::InnerClasses,
                    left
                )
            }
            "LineNumberTable" => {
                let (line_number_table, left) = left.read_line_number_table();
                (
                    AttributeInfo::LineNumberTable {
                        line_number_table
                    },
                    left
                )
            }
            "Local"
            _ => {
                panic!("");
            }
        }
    }

    fn read_attributes(&self, constant_pool: &ConstantPool) -> (Vec<AttributeInfo>, &[u8]) {
        let (attribute_info_amount, left) = self.read_u16();
        let mut attribute_info_vec = Vec::with_capacity(attribute_info_amount as usize);
        loopn!(attribute_info_amount, {
            let (attribute_info, left) = left.read_attribute(constant_pool);
            attribute_info_vec.push(attribute_info);
        });
        (attribute_info_vec, left)
    }

    fn parse(&self) -> ClassFile {
        let (_, left) = self.read_and_check_magic_number();
        let (VersionInfo { major_version, minor_version }, left) = left.read_and_check_version();
        let (constant_pool, left) = left.read_constant_pool();
        let (access_flags, left) = left.read_access_flags();
        let (this_class, left) = left.read_this_class();
        let (super_class, left) = left.read_super_class();
        let (interfaces, left) = left.read_interfaces();
        let (fields, left) = left.read_members(&constant_pool);
        let (methods, left) = left.read_members(&constant_pool);
        let (attributes, left) = left.read_attributes(&constant_pool);
        ClassFile {
            major_version,
            minor_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::class_loader::class_reader::ClassReader;

    #[test]
    pub fn test_read_u16s() {
        let u8_array: [u8; 10] = [0, 2, 1, 1, 0, 7, 0, 2, 1, 2];
        // result: 257  7   left 0 2 1 2
        let (result, left) = u8_array.read_u16s();
        println!("{:?}", result);
        println!();
        println!("{:?}", left);
    }
}

#[test]
pub fn test_loop_n() {
    // [1, 3]    include 1, 2, 3
    for _ in 1..=3 {
        println!("123")
    }
    println!();

    // [1,3)     include 1, 2
    for _ in 1..3 {
        println!("abc")
    }
    println!();

    // [1, 3]     include 1, 2, 3
    loopn!(3,{
        println!("666");
    });
}
