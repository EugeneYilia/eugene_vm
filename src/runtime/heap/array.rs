use std::rc::Rc;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::stack::variable_slot::VariableSlot;

#[derive(Debug)]
pub struct Array {
    class: Rc<Class>,
    values: Vec<VariableSlot>,
}