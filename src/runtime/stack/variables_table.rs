use std::collections::HashMap;

use crate::runtime::stack::variable_slot::VariableSlot;
use crate::util::instruction_util::variable_slot_type_is_kind_one;

#[derive(Debug)]
pub struct VariableTable {
    slot_map: HashMap<usize, VariableSlot>,
}

impl VariableTable {
    // 构建局部变量表的时候，将传递进来的参数variable_slot填充进去
    pub fn new(args: Option<Vec<VariableSlot>>) -> VariableTable {
        let mut slot_map = HashMap::new();
        if let Some(args) = args {
            let mut index = 1;
            args.iter().for_each(|variable_slot| {
                slot_map.insert(index, variable_slot.clone());
                if variable_slot_type_is_kind_one(variable_slot) {
                    index = index + 1;
                } else {
                    index = index + 2;
                }
            });
        }
        VariableTable {
            slot_map
        }
    }

    pub fn set_variable_slot(&mut self, index: usize, variable_slot: VariableSlot) {
        self.slot_map.insert(index, variable_slot);
    }

    pub fn get_variable_slot(&self, index: usize) -> &VariableSlot {
        match self.slot_map.get(&index) {
            Some(variable_slot) => variable_slot,
            _ => panic!("{} does not point to VariableSlot", index)
        }
    }

    pub fn get_variable_slot_mut(&mut self, index: usize) -> &mut VariableSlot {
        match self.slot_map.get_mut(&index) {
            Some(variable_slot) => variable_slot,
            _ => panic!("{} does not point to VariableSlot", index)
        }
    }
}