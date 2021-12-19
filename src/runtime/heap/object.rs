use std::collections::HashMap;
use std::rc::Rc;

use crate::runtime::heap::object_field::ObjectField;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::class::class_member::ClassMember;
use crate::runtime::stack::variable_slot::VariableSlot;

#[derive(Debug)]
pub struct Object {
    class: Rc<Class>,
    fields: HashMap<String, ObjectField>,
}