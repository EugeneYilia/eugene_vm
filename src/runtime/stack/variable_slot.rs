use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::env::var;
use std::num::Wrapping;
use std::ops::Deref;
use std::rc::Rc;

use crate::core::class_loader::class_loader::ClassLoader;
use crate::runtime::heap::array::Array;
use crate::runtime::heap::object::Object;
use crate::runtime::heap::object_field::ObjectField;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;
use crate::runtime::thread::Thread;

// 加了#[derive(Clone)] 自动实现了VariableSlot的clone而不只是struct引用的clone
// 局部变量 实例字段 类静态字段
#[derive(Debug, Clone)]
pub enum VariableSlot {
    I32(Wrapping<i32>),
    I64(Wrapping<i64>),
    F32(f32),
    F64(f64),
    ObjectReference(Rc<RefCell<Object>>),
    ArrayReference(Rc<RefCell<Array>>),
    TestReference(Rc<RefCell<Vec<String>>>),
    NullReference,
}

#[test]
fn show_all() {
    test_clone_a();
    test_clone_b();
}

#[test]
fn test_clone_a() {
    let mut variable_slot = VariableSlot::TestReference(Rc::new(RefCell::new(vec!["a".to_owned(), "d".to_owned()])));
    println!("variable_slot {:?}", variable_slot);
    let mut clone_variable_slot = variable_slot.clone();
    if let VariableSlot::TestReference(value) = clone_variable_slot {
        value.deref().borrow_mut().push("a".to_owned());
        println!("clone_variable_slot {:?}", value.deref().borrow_mut());
        println!("variable_slot {:?}", variable_slot);
    }
}

#[test]
fn test_clone_b() {
    let mut variable_slot = VariableSlot::I32(Wrapping(32i32));
    println!("variable_slot {:?}", variable_slot);
    let mut clone_variable_slot = variable_slot.clone();
    if let VariableSlot::I32(mut value) = clone_variable_slot {
        value = Wrapping(33i32);
        println!("clone_variable_slot {:?}", value);
        println!("variable_slot {:?}", variable_slot);
    }
}