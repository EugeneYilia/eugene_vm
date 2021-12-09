use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::core::class_loader::class_loader::ClassLoader;

use crate::runtime::method_area::class::field::Field;
use crate::runtime::method_area::class::method::Method;
use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;
use crate::runtime::stack::variables_table::VariableTable;
use crate::util::class_util::check_access_flags_all;

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
    pub fn get_method(&self, name: &str, descriptor: &str, access_flags: Vec<u16>) -> Rc<Method> {
        // Java main func:  public static void main(String[] args){}
        let method_ref = self.methods
            .iter()
            .find(|ref method| {
                method.get_name() == name &&
                    method.get_descriptor() == descriptor &&
                    check_access_flags_all(method.get_access_flags(), &access_flags)
            })
            .expect("Method not found");
        Rc::clone(method_ref)
    }
}