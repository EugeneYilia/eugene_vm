use std::collections::{BTreeMap, HashMap};
use std::ops::Deref;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;

// 常量池中主要存放2大类常量：字面量（Literal）和符号引用（Symbolic References）。
// 字面量主要指文本字符串、被声明为final的常量值。
#[derive(Debug)]
pub struct ConstantPool {
    pub constant_info_map: BTreeMap<usize, ConstantInfo>,
}

impl ConstantPool {
    pub fn insert(&mut self, index: usize, constant_info: ConstantInfo) {
        self.constant_info_map.insert(index, constant_info);
    }

    pub fn get(&self, index: usize) -> &ConstantInfo {
        self.constant_info_map.get(&index).expect(format!("Wrong index not match constant_info : {}", index).as_str())
    }

    pub fn capacity(&self) -> usize {
        self.constant_info_map.len()
    }

    fn get_modified_utf8(&self, index: usize) -> &str {
        match self.get(index) {
            ConstantInfo::ModifiedUTF8(ref name) => name.as_str(),
            _ => panic!("Wrong index not match utf8 : {}", index)
        }
    }

    /***
        index:  具体Class对应的index
        return: 使用index找到对应的class后根据name_index找到对应的name的&string 之后返回字面量只读视图&str
     */
    pub fn get_class_name(&self, class_index: usize) -> &str {
        let constant_info = self.get(class_index);
        match constant_info {
            ConstantInfo::Class { ref name_index } => self.get_modified_utf8(*name_index as usize),
            _ => panic!("Wrong index not match class: {}", class_index)
        }
    }
}