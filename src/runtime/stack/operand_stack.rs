use crate::runtime::stack::variable_slot::VariableSlot;

/// 操作数栈上不管是int 还是 long都是占据一个槽位的大小
/// long在操作数栈上占据一个槽位  但是long在局部变量表中占据两个槽位
/// 实现过程中  可以认为在局部变量表中也是一个槽位
#[derive(Debug)]
pub struct OperandStack {
    variable_slot_vec: Vec<VariableSlot>,
}

impl OperandStack {
    pub fn new(max_stack: usize) -> OperandStack {
        OperandStack {
            variable_slot_vec: Vec::with_capacity(max_stack)
        }
    }

    pub fn push_i32(&mut self, value: i32) {
        self.variable_slot_vec.push(VariableSlot::I32(value));
    }

    pub fn pop_i32(&mut self) -> i32 {
        let variable_slot = self.variable_slot_vec.pop().unwrap();
        if let VariableSlot::I32(value) = variable_slot {
            value
        } else {
            panic!("variable_slot: {:?} is not VariableSlot::I32", variable_slot);
        }
    }

    pub fn push_i64(&mut self, value: i64) {
        self.variable_slot_vec.push(VariableSlot::I64(value));
    }

    pub fn pop_i64(&mut self) -> i64 {
        let variable_slot = self.variable_slot_vec.pop().unwrap();
        if let VariableSlot::I64(value) = variable_slot {
            value
        } else {
            panic!("variable_slot: {:?} is not VariableSlot::I64", variable_slot);
        }
    }

    pub fn push_f32(&mut self, value: f32) {
        self.variable_slot_vec.push(VariableSlot::F32(value));
    }

    pub fn pop_f32(&mut self) -> f32 {
        let variable_slot = self.variable_slot_vec.pop().unwrap();
        if let VariableSlot::F32(value) = variable_slot {
            value
        } else {
            panic!("variable_slot: {:?} is not VariableSlot::F32", variable_slot);
        }
    }

    pub fn push_f64(&mut self, value: f64) {
        self.variable_slot_vec.push(VariableSlot::F64(value));
    }

    pub fn pop_f64(&mut self) -> f64 {
        let variable_slot = self.variable_slot_vec.pop().unwrap();
        if let VariableSlot::F64(value) = variable_slot {
            value
        } else {
            panic!("variable_slot: {:?} is not VariableSlot::F64", variable_slot);
        }
    }

    pub fn push(&mut self, variable_slot: VariableSlot) {
        self.variable_slot_vec.push(variable_slot);
    }

    pub fn pop(&mut self) -> VariableSlot {
        self.variable_slot_vec.pop().unwrap()
    }
}

#[test]
fn test_vec() {
    let mut vec = Vec::with_capacity(3);
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    println!("{:?}", vec);
}