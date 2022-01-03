use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::core::class_loader::class_loader::ClassLoader;
use crate::runtime::heap::object_field::ObjectField;
use crate::runtime::method_area::class::field::Field;
use crate::runtime::method_area::class::method::Method;
use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;
use crate::runtime::stack::variables_table::VariableTable;
use crate::util::class_util::check_access_flags_all;
use crate::util::instruction_util::get_default_variable_slot;

// class的生命周期要长于class_loader
#[derive(Debug)]
pub struct Class {
    pub access_flags: u16,
    pub constant_pool: ConstantPool,
    pub class_name: String,
    pub fields: Vec<Field>,
    pub methods: Vec<Rc<Method>>,
    pub super_class: Option<Rc<Class>>,
    // fields中一个field不一定占据一个或者两个slot  需要记录下接下来应该分配的next_instance_slot_id 和 next_static_slot_id
    pub next_instance_slot_id: usize,
    pub next_static_slot_id: usize,
    pub static_variable_table: VariableTable,
    pub class_loader: Option<Rc<RefCell<ClassLoader>>>,
}

impl Class {
    pub fn get_method(&self, name: &str, descriptor: &str, access_flags: Vec<u16>) -> Option<Rc<Method>> {
        // Java main func:  public static void main(String[] args){}
        self.methods
            .iter()
            .find(|ref method| {
                method.get_name() == name &&
                    method.get_descriptor() == descriptor &&
                    check_access_flags_all(method.get_access_flags(), &access_flags)
            }).map(|method| Rc::clone(method))
    }

    pub fn collect_instance_fields(&self, mut fields: HashMap<String, ObjectField>) -> HashMap<String, ObjectField> {
        if let Some(super_class) = &self.super_class {
            fields = super_class.collect_instance_fields(fields)
        }

        self.fields.iter().for_each(|field| {
            fields.insert(field.get_name().to_owned(), ObjectField::new(field.get_class_member(), get_default_variable_slot(field.get_descriptor())));
        });

        fields
    }
}