use crate::runtime::method_area::class::class_member::ClassMember;
use crate::runtime::stack::variable_slot::VariableSlot;

#[derive(Debug)]
pub struct ObjectField {
    pub class_member: ClassMember,
    pub variable_slot: VariableSlot,
}