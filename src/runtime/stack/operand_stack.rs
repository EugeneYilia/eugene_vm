use crate::runtime::stack::variable_slot::VariableSlot;
use crate::util::converter;

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
        match variable_slot {
            VariableSlot::I32(value) => value,
        }
    }

    pub fn push_i64(&mut self, value: i64) {
        let [first, second] = converter::i64_to_i32seq(value);
        self.push_i32(first);
        self.push_i32(second);
    }

    pub fn pop_i64(&mut self) -> i64 {
        let second = self.pop_i32();
        let first = self.pop_i32();
        converter::i32seq_to_i64([first, second])
    }

    pub fn push_f32(&mut self, value: f32) {
        let value = converter::f32_to_i32(value);
        self.push_i32(value)
    }

    pub fn pop_f32(&mut self) -> f32 {
        let i32_value = self.pop_i32();
        converter::i32_to_f32(i32_value)
    }

    pub fn push_f64(&mut self, value: f64) {
        let [first, second] = converter::f64_to_i32seq(value);
        self.push_i32(first);
        self.push_i32(second);
    }

    pub fn pop_f64(&mut self) -> f64 {
        let second = self.pop_i32();
        let first = self.pop_i32();
        converter::i32seq_to_f64([first, second])
    }
}