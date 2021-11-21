use std::collections::BTreeMap;

use byteorder::{BigEndian, ByteOrder};

use crate::constants::constant_info_tag::*;
use crate::core::classfile::attribute_info::attribute_info::AttributeInfo;
use crate::core::classfile::attribute_info::exception_table_entry::ExceptionTableEntry;
use crate::core::classfile::attribute_info::line_number_table_entry::LineNumberTableEntry;
use crate::core::classfile::attribute_info::local_variable_table_entry::LocalVariableTableEntry;
use crate::core::classfile::classfile::ClassFile;
use crate::core::classfile::member_info::MemberInfo;
use crate::core::classfile::version_info::VersionInfo;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;

pub trait ClassReader {
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

                (ConstantInfo::ModifiedUTF8(value), left)
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
        let (can_not_reach_start_index, mut left) = self.read_u16();
        let mut constant_pool = ConstantPool { constant_info_map: BTreeMap::new() };

        let mut constant_info_index: usize = 1;
        while constant_info_index < (can_not_reach_start_index as usize) {
            let (constant_info, current_left) = left.read_constant_info();
            left = current_left;
            let add_amount = match constant_info {
                ConstantInfo::Long(_) | ConstantInfo::Double(_) => 2,
                _ => 1
            };
            constant_pool.insert(constant_info_index, constant_info);
            constant_info_index += add_amount;
        }
        (constant_pool, left)
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
            ConstantInfo::ModifiedUTF8(ref name) => name.to_owned(),
            _ => panic!("name_index doesn't point to UTF8 String")
        };
        let descriptor = match constant_pool.get(descriptor_index as usize) {
            ConstantInfo::ModifiedUTF8(ref name) => name.to_owned(),
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
        let (member_info_amount, mut left) = self.read_u16();
        let mut member_info_vec = Vec::with_capacity(member_info_amount as usize);
        loopn!(member_info_amount,{
            let (member_info, u8_left) = left.read_member(constant_pool);
            left = u8_left;
            member_info_vec.push(member_info);
        });
        (member_info_vec, left)
    }

    fn read_exception_table(&self) -> (Vec<ExceptionTableEntry>, &[u8]) {
        let (exception_table_length, mut left) = self.read_u16();
        let mut exception_table: Vec<ExceptionTableEntry> = Vec::with_capacity(exception_table_length as usize);
        loopn!(exception_table_length, {
            let (start_pc, u8_left) = left.read_u16();
            let (end_pc, u8_left) = u8_left.read_u16();
            let (handle_pc, u8_left) = u8_left.read_u16();
            let (catch_type, u8_left) = u8_left.read_u16();
            left = u8_left;
            let exception_table_entry = ExceptionTableEntry{ start_pc, end_pc, handle_pc, catch_type };
            exception_table.push(exception_table_entry);
        });
        (exception_table, left)
    }

    fn read_line_number_table(&self) -> (Vec<LineNumberTableEntry>, &[u8]) {
        let (line_number_table_length, mut left) = self.read_u16();
        let mut line_number_table: Vec<LineNumberTableEntry> = Vec::with_capacity(line_number_table_length as usize);
        loopn!(line_number_table_length, {
            let (start_pc, u8_left) = left.read_u16();
            let (line_number, u8_left) = u8_left.read_u16();
            left = u8_left;
            let line_number_table_entry = LineNumberTableEntry{ start_pc, line_number };
            line_number_table.push(line_number_table_entry)
        });
        (line_number_table, left)
    }

    fn read_local_variable_table(&self) -> (Vec<LocalVariableTableEntry>, &[u8]) {
        let (local_variable_table_length, mut left) = self.read_u16();
        let mut local_variable_table = Vec::with_capacity(local_variable_table_length as usize);
        loopn!(local_variable_table_length, {
            let (start_pc, u8_left) = left.read_u16();
            let (length, u8_left) = u8_left.read_u16();
            let (name_index, u8_left) = u8_left.read_u16();
            let (descriptor_index, u8_left) = u8_left.read_u16();
            let (index, u8_left) = u8_left.read_u16();
            left = u8_left;
            let local_variable_table_entry = LocalVariableTableEntry{ start_pc, length, name_index, descriptor_index, index};
            local_variable_table.push(local_variable_table_entry);
        });
        (local_variable_table, left)
    }

    fn read_attribute(&self, constant_pool: &ConstantPool) -> (AttributeInfo, &[u8]) {
        let (attribute_name_index, left) = self.read_u16();
        let attribute_name = match constant_pool.get(attribute_name_index as usize) {
            ConstantInfo::ModifiedUTF8(attribute_name) => attribute_name,
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
            "LineNumberTable" => {
                let (line_number_table, left) = left.read_line_number_table();
                (
                    AttributeInfo::LineNumberTable {
                        line_number_table
                    },
                    left
                )
            }
            "LocalVariableTable" => {
                let (local_variable_table, left) = left.read_local_variable_table();
                (
                    AttributeInfo::LocalVariableTable {
                        local_variable_table
                    },
                    left
                )
            }
            "SourceFile" => {
                let (source_file_index, left) = left.read_u16();
                (
                    AttributeInfo::SourceFile {
                        source_file_index
                    },
                    left
                )
            }
            "Synthetic" => {
                (
                    AttributeInfo::Synthetic,
                    left
                )
            }
            _ => {
                let (_, left) = left.read_bytes(attribute_length as usize);
                (
                    AttributeInfo::Unparsed {
                        attribute_name: attribute_name.to_string(),
                        attribute_length,
                    },
                    left
                )
            }
        }
    }

    fn read_attributes(&self, constant_pool: &ConstantPool) -> (Vec<AttributeInfo>, &[u8]) {
        let (attribute_info_amount, mut left) = self.read_u16();
        let mut attribute_info_vec = Vec::with_capacity(attribute_info_amount as usize);
        loopn!(attribute_info_amount, {
            let (attribute_info, u8_left) = left.read_attribute(constant_pool);
            left = u8_left;
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
        let (attributes, _left) = left.read_attributes(&constant_pool);
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

// javap -v Object.class
//   Last modified 2020年11月6日; size 1497 bytes
//   SHA-256 checksum 91e9554ea0fac57b7ca9595ab777fe2578af2e5b111e8c240e1793f1a3686292
//   Compiled from "Object.java"
// public class java.lang.Object
//   minor version: 0
//   major version: 52
//   flags: (0x0021) ACC_PUBLIC, ACC_SUPER
//   this_class: #17                         // java/lang/Object
//   super_class: #0
//   interfaces: 0, fields: 0, methods: 14, attributes: 1
// Constant pool:
//    #1 = Class              #49            // java/lang/StringBuilder
//    #2 = Methodref          #1.#50         // java/lang/StringBuilder."<init>":()V
//    #3 = Methodref          #17.#51        // java/lang/Object.getClass:()Ljava/lang/Class;
//    #4 = Methodref          #52.#53        // java/lang/Class.getName:()Ljava/lang/String;
//    #5 = Methodref          #1.#54         // java/lang/StringBuilder.append:(Ljava/lang/String;)Ljava/lang/StringBuilder;
//    #6 = String             #55            // @
//    #7 = Methodref          #17.#56        // java/lang/Object.hashCode:()I
//    #8 = Methodref          #57.#58        // java/lang/Integer.toHexString:(I)Ljava/lang/String;
//    #9 = Methodref          #1.#59         // java/lang/StringBuilder.toString:()Ljava/lang/String;
//   #10 = Class              #60            // java/lang/IllegalArgumentException
//   #11 = String             #61            // timeout value is negative
//   #12 = Methodref          #10.#62        // java/lang/IllegalArgumentException."<init>":(Ljava/lang/String;)V
//   #13 = Integer            999999
//   #14 = String             #63            // nanosecond timeout value out of range
//   #15 = Methodref          #17.#64        // java/lang/Object.wait:(J)V
//   #16 = Methodref          #17.#65        // java/lang/Object.registerNatives:()V
//   #17 = Class              #66            // java/lang/Object
//   #18 = Utf8               <init>
//   #19 = Utf8               ()V
//   #20 = Utf8               Code
//   #21 = Utf8               LineNumberTable
//   #22 = Utf8               registerNatives
//   #23 = Utf8               getClass
//   #24 = Utf8               ()Ljava/lang/Class;
//   #25 = Utf8               Signature
//   #26 = Utf8               ()Ljava/lang/Class<*>;
//   #27 = Utf8               hashCode
//   #28 = Utf8               ()I
//   #29 = Utf8               equals
//   #30 = Utf8               (Ljava/lang/Object;)Z
//   #31 = Utf8               StackMapTable
//   #32 = Utf8               clone
//   #33 = Utf8               ()Ljava/lang/Object;
//   #34 = Utf8               Exceptions
//   #35 = Class              #67            // java/lang/CloneNotSupportedException
//   #36 = Utf8               toString
//   #37 = Utf8               ()Ljava/lang/String;
//   #38 = Utf8               notify
//   #39 = Utf8               notifyAll
//   #40 = Utf8               wait
//   #41 = Utf8               (J)V
//   #42 = Class              #68            // java/lang/InterruptedException
//   #43 = Utf8               (JI)V
//   #44 = Utf8               finalize
//   #45 = Class              #69            // java/lang/Throwable
//   #46 = Utf8               <clinit>
//   #47 = Utf8               SourceFile
//   #48 = Utf8               Object.java
//   #49 = Utf8               java/lang/StringBuilder
//   #50 = NameAndType        #18:#19        // "<init>":()V
//   #51 = NameAndType        #23:#24        // getClass:()Ljava/lang/Class;
//   #52 = Class              #70            // java/lang/Class
//   #53 = NameAndType        #71:#37        // getName:()Ljava/lang/String;
//   #54 = NameAndType        #72:#73        // append:(Ljava/lang/String;)Ljava/lang/StringBuilder;
//   #55 = Utf8               @
//   #56 = NameAndType        #27:#28        // hashCode:()I
//   #57 = Class              #74            // java/lang/Integer
//   #58 = NameAndType        #75:#76        // toHexString:(I)Ljava/lang/String;
//   #59 = NameAndType        #36:#37        // toString:()Ljava/lang/String;
//   #60 = Utf8               java/lang/IllegalArgumentException
//   #61 = Utf8               timeout value is negative
//   #62 = NameAndType        #18:#77        // "<init>":(Ljava/lang/String;)V
//   #63 = Utf8               nanosecond timeout value out of range
//   #64 = NameAndType        #40:#41        // wait:(J)V
//   #65 = NameAndType        #22:#19        // registerNatives:()V
//   #66 = Utf8               java/lang/Object
//   #67 = Utf8               java/lang/CloneNotSupportedException
//   #68 = Utf8               java/lang/InterruptedException
//   #69 = Utf8               java/lang/Throwable
//   #70 = Utf8               java/lang/Class
//   #71 = Utf8               getName
//   #72 = Utf8               append
//   #73 = Utf8               (Ljava/lang/String;)Ljava/lang/StringBuilder;
//   #74 = Utf8               java/lang/Integer
//   #75 = Utf8               toHexString
//   #76 = Utf8               (I)Ljava/lang/String;
//   #77 = Utf8               (Ljava/lang/String;)V
// {
//   public java.lang.Object();
//     descriptor: ()V
//     flags: (0x0001) ACC_PUBLIC
//     Code:
//       stack=0, locals=1, args_size=1
//          0: return
//       LineNumberTable:
//         line 37: 0
//
//   public final native java.lang.Class<?> getClass();
//     descriptor: ()Ljava/lang/Class;
//     flags: (0x0111) ACC_PUBLIC, ACC_FINAL, ACC_NATIVE
//     Signature: #26                          // ()Ljava/lang/Class<*>;
//
//   public native int hashCode();
//     descriptor: ()I
//     flags: (0x0101) ACC_PUBLIC, ACC_NATIVE
//
//   public boolean equals(java.lang.Object);
//     descriptor: (Ljava/lang/Object;)Z
//     flags: (0x0001) ACC_PUBLIC
//     Code:
//       stack=2, locals=2, args_size=2
//          0: aload_0
//          1: aload_1
//          2: if_acmpne     9
//          5: iconst_1
//          6: goto          10
//          9: iconst_0
//         10: ireturn
//       LineNumberTable:
//         line 149: 0
//       StackMapTable: number_of_entries = 2
//         frame_type = 9 /* same */
//         frame_type = 64 /* same_locals_1_stack_item */
//           stack = [ int ]
//
//   protected native java.lang.Object clone() throws java.lang.CloneNotSupportedException;
//     descriptor: ()Ljava/lang/Object;
//     flags: (0x0104) ACC_PROTECTED, ACC_NATIVE
//     Exceptions:
//       throws java.lang.CloneNotSupportedException
//
//   public java.lang.String toString();
//     descriptor: ()Ljava/lang/String;
//     flags: (0x0001) ACC_PUBLIC
//     Code:
//       stack=2, locals=1, args_size=1
//          0: new           #1                  // class java/lang/StringBuilder
//          3: dup
//          4: invokespecial #2                  // Method java/lang/StringBuilder."<init>":()V
//          7: aload_0
//          8: invokevirtual #3                  // Method getClass:()Ljava/lang/Class;
//         11: invokevirtual #4                  // Method java/lang/Class.getName:()Ljava/lang/String;
//         14: invokevirtual #5                  // Method java/lang/StringBuilder.append:(Ljava/lang/String;)Ljava/lang/StringBuilder;
//         17: ldc           #6                  // String @
//         19: invokevirtual #5                  // Method java/lang/StringBuilder.append:(Ljava/lang/String;)Ljava/lang/StringBuilder;
//         22: aload_0
//         23: invokevirtual #7                  // Method hashCode:()I
//         26: invokestatic  #8                  // Method java/lang/Integer.toHexString:(I)Ljava/lang/String;
//         29: invokevirtual #5                  // Method java/lang/StringBuilder.append:(Ljava/lang/String;)Ljava/lang/StringBuilder;
//         32: invokevirtual #9                  // Method java/lang/StringBuilder.toString:()Ljava/lang/String;
//         35: areturn
//       LineNumberTable:
//         line 236: 0
//
//   public final native void notify();
//     descriptor: ()V
//     flags: (0x0111) ACC_PUBLIC, ACC_FINAL, ACC_NATIVE
//
//   public final native void notifyAll();
//     descriptor: ()V
//     flags: (0x0111) ACC_PUBLIC, ACC_FINAL, ACC_NATIVE
//
//   public final native void wait(long) throws java.lang.InterruptedException;
//     descriptor: (J)V
//     flags: (0x0111) ACC_PUBLIC, ACC_FINAL, ACC_NATIVE
//     Exceptions:
//       throws java.lang.InterruptedException
//
//   public final void wait(long, int) throws java.lang.InterruptedException;
//     descriptor: (JI)V
//     flags: (0x0011) ACC_PUBLIC, ACC_FINAL
//     Code:
//       stack=4, locals=4, args_size=3
//          0: lload_1
//          1: lconst_0
//          2: lcmp
//          3: ifge          16
//          6: new           #10                 // class java/lang/IllegalArgumentException
//          9: dup
//         10: ldc           #11                 // String timeout value is negative
//         12: invokespecial #12                 // Method java/lang/IllegalArgumentException."<init>":(Ljava/lang/String;)V
//         15: athrow
//         16: iload_3
//         17: iflt          26
//         20: iload_3
//         21: ldc           #13                 // int 999999
//         23: if_icmple     36
//         26: new           #10                 // class java/lang/IllegalArgumentException
//         29: dup
//         30: ldc           #14                 // String nanosecond timeout value out of range
//         32: invokespecial #12                 // Method java/lang/IllegalArgumentException."<init>":(Ljava/lang/String;)V
//         35: athrow
//         36: iload_3
//         37: ifle          44
//         40: lload_1
//         41: lconst_1
//         42: ladd
//         43: lstore_1
//         44: aload_0
//         45: lload_1
//         46: invokevirtual #15                 // Method wait:(J)V
//         49: return
//       LineNumberTable:
//         line 447: 0
//         line 448: 6
//         line 451: 16
//         line 452: 26
//         line 456: 36
//         line 457: 40
//         line 460: 44
//         line 461: 49
//       StackMapTable: number_of_entries = 4
//         frame_type = 16 /* same */
//         frame_type = 9 /* same */
//         frame_type = 9 /* same */
//         frame_type = 7 /* same */
//     Exceptions:
//       throws java.lang.InterruptedException
//
//   public final void wait() throws java.lang.InterruptedException;
//     descriptor: ()V
//     flags: (0x0011) ACC_PUBLIC, ACC_FINAL
//     Code:
//       stack=3, locals=1, args_size=1
//          0: aload_0
//          1: lconst_0
//          2: invokevirtual #15                 // Method wait:(J)V
//          5: return
//       LineNumberTable:
//         line 502: 0
//         line 503: 5
//     Exceptions:
//       throws java.lang.InterruptedException
//
//   protected void finalize() throws java.lang.Throwable;
//     descriptor: ()V
//     flags: (0x0004) ACC_PROTECTED
//     Code:
//       stack=0, locals=1, args_size=1
//          0: return
//       LineNumberTable:
//         line 555: 0
//     Exceptions:
//       throws java.lang.Throwable
//
//   static {};
//     descriptor: ()V
//     flags: (0x0008) ACC_STATIC
//     Code:
//       stack=0, locals=0, args_size=0
//          0: invokestatic  #16                 // Method registerNatives:()V
//          3: return
//       LineNumberTable:
//         line 41: 0
//         line 42: 3
// }
// SourceFile: "Object.java"
// 测试直接从文件里读取和从jar包里读取对应的class文件
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use crate::core::class_loader::class_reader::ClassReader;
    use crate::core::classfile::attribute_info::attribute_info::AttributeInfo;
    use crate::core::classfile::classfile::ClassFile;
    use crate::core::classfile::member_info::MemberInfo;
    use crate::core::classpath::classpath_entry::ClasspathEntry;
    use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;

    fn panic_type_not_match(index: usize, err_msg: &str) {
        panic!("Index {} constant_info type is not {}", index, err_msg);
    }

    #[test]
    fn parse_from_file() {
        // 直接文件读取
        let path: &str = "eugene_test/byte_code/rt/java/lang/Object.class";
        let file = File::open(path).unwrap();
        let file_bytes: Vec<u8> = file.bytes().map(|result_u8| result_u8.unwrap()).collect();

        check_class_file(file_bytes);
    }

    #[test]
    fn parse_from_dir() {
        let classpath_entry_dir = ClasspathEntry::new("eugene_test/byte_code/rt");
        let file_bytes = classpath_entry_dir.read_class("java/lang/Object.class").unwrap();

        check_class_file(file_bytes);
    }

    #[test]
    fn parse_from_jar() {
        let classpath_entry_jar = ClasspathEntry::new("eugene_test/byte_code/rt.jar");
        let file_bytes = classpath_entry_jar.read_class("java/lang/Object.class").unwrap();

        check_class_file(file_bytes);
    }

    #[test]
    fn parse_from_wildcard() {
        let classpath_entry_wildcard = ClasspathEntry::new("eugene_test/byte_code/*");
        let file_bytes = classpath_entry_wildcard.read_class("java/lang/Object.class").unwrap();

        check_class_file(file_bytes);
    }

    fn check_class_file(file_bytes: Vec<u8>) {
        let class_file = file_bytes.parse();
        println!("{:?}", class_file);
        let ClassFile {
            major_version,
            minor_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes
        } = class_file;
        assert_eq!(major_version, 52u16);
        assert_eq!(minor_version, 0u16);
        assert_eq!(constant_pool.capacity(), 77usize);
        match constant_pool.get(1usize) {
            ConstantInfo::Class { name_index } => assert_eq!(*name_index, 49u16),
            _ => panic_type_not_match(1usize, "ConstantInfo::Class")
        };
        match constant_pool.get(9usize) {
            ConstantInfo::MethodRef { class_index, name_and_type_index } => {
                assert_eq!(class_index.to_owned(), 1u16);
                assert_eq!(name_and_type_index.to_owned(), 59u16);
            }
            _ => panic_type_not_match(9usize, "ConstantInfo::MethodRef")
        };
        match constant_pool.get(13usize) {
            ConstantInfo::Integer(value) => assert_eq!(*value, 999999i32),
            _ => panic_type_not_match(13usize, "ConstantInfo::Integer")
        };
        match constant_pool.get(22usize) {
            ConstantInfo::ModifiedUTF8(value) => assert_eq!(value, "registerNatives"),
            _ => panic_type_not_match(22usize, "ConstantInfo::ModifiedUTF8")
        };
        match constant_pool.get(50usize) {
            ConstantInfo::NameAndType { name_index, descriptor_index } => {
                assert_eq!(*name_index, 18u16);
                assert_eq!(*descriptor_index, 19u16);
            }
            _ => panic_type_not_match(50usize, "ConstantInfo::NameAndType")
        };
        match constant_pool.get(77usize) {
            ConstantInfo::ModifiedUTF8(value) => assert_eq!(value, "(Ljava/lang/String;)V"),
            _ => panic_type_not_match(77usize, "ConstantInfo::ModifiedUTF8")
        };
        assert_eq!(access_flags, 33u16);
        assert_eq!(this_class, 17u16);
        assert_eq!(super_class, 0u16);
        assert_eq!(interfaces.len(), 0usize);
        assert_eq!(fields.len(), 0usize);
        assert_eq!(methods.len(), 14usize);

        assert_eq!(attributes.len(), 1usize);
        match attributes.get(0).unwrap() {
            AttributeInfo::SourceFile { source_file_index } => {
                assert_eq!(*source_file_index, 48u16)
            }
            _ => panic!("classfile attribute 0 is not AttributeInfo::SourceFile")
        }

        println!("methods member_info:");
        methods.iter().for_each(|method| println!("{:?}", method));
        let MemberInfo {
            access_flags,
            name: _,
            name_index,
            descriptor: _,
            descriptor_index,
            attributes
        } = methods.get(2).unwrap();
        assert_eq!(*name_index, 23u16);
        assert_eq!(*descriptor_index, 24u16);
        assert_eq!(attributes.len(), 1usize);
        assert_eq!(*access_flags, 273u16);

        let MemberInfo {
            access_flags,
            name: _,
            name_index,
            descriptor: _,
            descriptor_index,
            attributes
        } = methods.get(12).unwrap();
        assert_eq!(*name_index, 44u16);
        assert_eq!(*descriptor_index, 19u16);
        assert_eq!(attributes.len(), 2usize);// finalize    [0]Code  [1]Exceptions
        assert_eq!(*access_flags, 4u16);

        let MemberInfo {
            access_flags,
            name: _,
            name_index,
            descriptor: _,
            descriptor_index,
            attributes
        } = methods.get(13).unwrap();
        assert_eq!(*name_index, 46u16);
        assert_eq!(*descriptor_index, 19u16);
        assert_eq!(attributes.len(), 1usize);
        assert_eq!(*access_flags, 8u16);
        match attributes.get(0).unwrap() {
            AttributeInfo::Code {
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes
            } => {
                assert_eq!(*max_stack, 0u16);
                assert_eq!(*max_locals, 0u16);
                assert_eq!(code.len(), 4usize);
                assert_eq!(exception_table.len(), 0usize);
                assert_eq!(attributes.len(), 1usize);
                match attributes.get(0).unwrap() {
                    AttributeInfo::LineNumberTable {
                        line_number_table
                    } => {
                        assert_eq!(line_number_table.len(), 2usize);

                        let line_number_table_entry_0 = line_number_table.get(0).unwrap();
                        assert_eq!(line_number_table_entry_0.start_pc, 0u16);
                        assert_eq!(line_number_table_entry_0.line_number, 41u16);

                        let line_number_table_entry_1 = line_number_table.get(1).unwrap();
                        assert_eq!(line_number_table_entry_1.start_pc, 3u16);
                        assert_eq!(line_number_table_entry_1.line_number, 42u16);
                    }
                    _ => panic!("Method 13 attribute 0 attribute 0 is not AttributeInfo::LineNumberTable")
                }
            }
            _ => panic!("Method 13 attribute 0 is not AttributeInfo::Code")
        }
    }

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
