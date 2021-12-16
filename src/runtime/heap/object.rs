use crate::runtime::method_area::class::class::Class;
use crate::runtime::stack::variable_slot::VariableSlot;

#[derive(Debug)]
pub struct Object {
    class: Class,
    fields: Vec<VariableSlot>,
}