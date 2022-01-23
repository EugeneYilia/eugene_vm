use std::rc::Rc;

use crate::runtime::method_area::class::class_member::ClassMember;
use crate::runtime::stack::variable_slot::VariableSlot;

// 类Class对象的字段   类实例Class Instance对象的字段
#[derive(Debug)]
pub struct ObjectField {
    pub class_member: Rc<ClassMember>,
    pub variable_slot: VariableSlot,
}

impl ObjectField {
    pub fn new(class_member: Rc<ClassMember>, variable_slot: VariableSlot) -> ObjectField {
        ObjectField {
            class_member,
            variable_slot,
        }
    }
}