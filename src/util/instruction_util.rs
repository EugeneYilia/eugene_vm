use crate::constants::instruction_constants::TERMINAL_INSTRUCTION_OPCODES;
use crate::runtime::stack::variable_slot::VariableSlot;

/// 实际类型	计算型	类别
/// boolean	int	1
/// byte	int	1
/// char	int	1
/// short	int	1
/// int	int	1
/// float	float	1
/// reference	reference	1
/// returnAddress	returnAddress	1
/// long	long	2
/// double	double	2
pub fn variable_slot_type_is_kind_one(variable_slot: &VariableSlot) -> bool {
    match variable_slot {
        VariableSlot::I64(_) | VariableSlot::F64(_) => false,
        _ => true
    }
}

pub fn is_terminal_instruction(instruction_op_code: &u8) -> bool {
    TERMINAL_INSTRUCTION_OPCODES.contains(instruction_op_code)
}