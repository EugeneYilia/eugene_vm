use std::collections::HashMap;
use crate::runtime::stack::variable_slot::VariableSlot;

#[derive(Debug)]
pub struct VariableTable {
    slot_map: HashMap<usize, VariableSlot>,
}

impl VariableTable {
    pub fn new() -> VariableTable {
        VariableTable {
            slot_map: HashMap::new()
        }
    }

    // 链式调用
    pub fn set_variable_slot(&mut self, index: usize, variable_slot: VariableSlot) {
        self.slot_map.insert(index, variable_slot);
    }

    pub fn get_variable_slot(&self, index: usize) -> &VariableSlot {
        match self.slot_map.get(&index) {
            Some(variable_slot) => variable_slot,
            _ => panic!("{} does not point to VariableSlot", index)
        }
    }
}