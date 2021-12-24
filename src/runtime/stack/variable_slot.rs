use std::cell::RefCell;
use std::rc::Rc;

use crate::runtime::heap::array::Array;
use crate::runtime::heap::object::Object;

// 局部变量 实例字段 类静态字段
#[derive(Debug)]
pub enum VariableSlot {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    ObjectReference(Rc<RefCell<Object>>),
    ArrayReference(Rc<RefCell<Array>>),
    NullReference,
}