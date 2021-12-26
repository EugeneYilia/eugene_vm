use std::cell::RefCell;
use std::num::Wrapping;
use std::rc::Rc;

use crate::runtime::heap::array::Array;
use crate::runtime::heap::object::Object;

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
    NullReference,
}