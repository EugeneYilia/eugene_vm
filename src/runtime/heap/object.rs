use std::collections::HashMap;
use std::rc::Rc;

use crate::runtime::heap::object_field::ObjectField;
use crate::runtime::method_area::class::class::Class;

#[derive(Debug)]
pub struct Object {
    pub class: Rc<Class>,
    pub fields: HashMap<String, ObjectField>,
}

impl Object {
    pub fn new(class: Rc<Class>) -> Object {
        Object {
            class: Rc::clone(&class),
            fields: class.collect_instance_fields(HashMap::<String, ObjectField>::new()),
        }
    }
}