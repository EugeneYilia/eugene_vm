use std::rc::Rc;

use crate::constants::descriptor::*;
use crate::core::classfile::attribute_info::attribute_info::AttributeInfo;
use crate::core::classfile::member_info::MemberInfo;
use crate::runtime::method_area::class::class_member::ClassMember;
use crate::runtime::stack::variable_slot::VariableSlot;
use crate::util::instruction_util::get_default_variable_slot;

// access_flags   descriptor      name        descriptor
//  public          String       getName    (String name){}
#[derive(Debug)]
pub struct Method {
    class_member: ClassMember,
    pub max_stack: usize,
    pub max_locals: usize,
    pub code: Rc<Vec<u8>>,
    pub args_type: Option<Vec<VariableSlot>>,
}

impl Method {
    pub fn new(member_info: &MemberInfo) -> Method {
        let class_member = ClassMember::new(member_info);
        let attribute_info = member_info.get_attribute_code();
        match attribute_info {
            Some(AttributeInfo::Code {
                     max_stack,
                     max_locals,
                     code,
                     ..
                 }) => {
                Method {
                    class_member,
                    max_stack: *max_stack as usize,
                    max_locals: *max_locals as usize,
                    code: Rc::new(code.clone()),
                    args_type: Method::parse_args_type(member_info.descriptor.as_str()),
                }
            }
            None => {
                // 抽象方法  接口方法  native方法
                Method {
                    class_member,
                    max_stack: 0usize,
                    max_locals: 0usize,
                    code: Rc::new(Vec::new()),
                    args_type: Method::parse_args_type(member_info.descriptor.as_str()),
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    // 1. 先定位出来) 然后就可以判断要不要创建初始Vec   2. 创建好Vec根据具体的元素数量来返回None还是Some(Vec)   目前选择第一种
    fn parse_args_type(descriptor: &str) -> Option<Vec<VariableSlot>> {
        let mut end_index_option = descriptor.find(")");
        let end_index: usize;
        if let Some(end_index_value) = end_index_option {
            if end_index_value == 1 {
                return None;
            } else {
                end_index = end_index_value;
            }
        } else {
            panic!("not valid method descriptor: {}", descriptor);
        }
        // 0 index -> (
        let mut current_index = 1;
        let mut arg_type_vec: Vec<VariableSlot> = vec![];

        loop {
            if current_index == end_index {
                return Some(arg_type_vec);
            }
            let current_char = indexn_char!(descriptor, current_index);

            // todo!("或许可以优化下这里  Obj和Array类型使用更准确的类型声明")
            arg_type_vec.push(get_default_variable_slot(current_char));
            match current_char {
                BYTE_DESCRIPTOR | CHAR_DESCRIPTOR | INT_DESCRIPTOR | SHORT_DESCRIPTOR | BOOLEAN_DESCRIPTOR
                | LONG_DESCRIPTOR
                | FLOAT_DESCRIPTOR
                | DOUBLE_DESCRIPTOR => {
                    current_index += 1;
                }
                OBJ_DESCRIPTOR | ARRAY_DESCRIPTOR => {
                    current_index += 1;
                    loop {
                        let current_char = indexn_char!(descriptor, current_index);
                        if current_char == ';' {
                            current_index += 1;
                            break;
                        } else {
                            current_index += 1;
                        }
                    }
                }
                _ => {
                    panic!("invalid method arg descriptor: {}  position: {}", current_char, current_index)
                }
            }
        }
    }

    pub fn get_name(&self) -> &str {
        self.class_member.name.as_str()
    }

    pub fn get_descriptor(&self) -> &str {
        self.class_member.descriptor.as_str()
    }

    pub fn get_access_flags(&self) -> u16 {
        self.class_member.access_flags
    }
}

#[test]
fn test_find_char() {
    let source = "()I";
    println!("{:?}", source.find(")"));
    println!("{:?}", source.find("I"));
    println!("{:?}", &source[1..2]);
}