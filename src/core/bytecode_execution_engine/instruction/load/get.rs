use std::ops::Deref;
use std::rc::Rc;
use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::class_loader::class_loader::ClassLoader;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn get_static(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let static_field_index = code_reader.read_u16() as usize;
    println!("static_field_index: {}", static_field_index);
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { class, .. } = stack_frame;
    let class = class.clone();
    if let ConstantInfo::FieldRef { class_index, name_and_type_index } = class.constant_pool.get(static_field_index) {
        println!("class_index: {}", class_index);
        println!("name_and_type_index: {}", name_and_type_index);
        if let ConstantInfo::Class { name_index } = class.constant_pool.get(*class_index as usize) {
            println!("name_index: {}", name_index);
            if let ConstantInfo::ModifiedUTF8(ref class_name) = class.constant_pool.get(*name_index as usize) {
                println!("class_name: {}", class_name);
                let Class { class_loader, .. } = class.deref();
                if let Some(class_loader) = class_loader {
                    let class_ref = ClassLoader::load_class(Rc::clone(class_loader), class_name.to_owned());
                    if let ConstantInfo::NameAndType{name_index, descriptor_index} = class.constant_pool.get(*name_and_type_index as usize) {
                        if let ConstantInfo::ModifiedUTF8(field_name) = class.constant_pool.get(*name_index as usize) {
                            if let ConstantInfo::ModifiedUTF8(field_descriptor) = class.constant_pool.get(*descriptor_index as usize) {
                                println!("static field field_name: {}  field_descriptor: {}", field_name, field_descriptor);
                            } else {
                                panic!("name_and_type_index: {}  descriptor_index: {} should point to ConstantInfo::ModifiedUTF8", name_and_type_index, descriptor_index);
                            }
                        } else {
                            panic!("name_and_type_index: {}  name_index: {} should point to ConstantInfo::ModifiedUTF8", name_and_type_index, name_index);
                        }
                    } else {
                        panic!("name_and_type_index: {} should point to ConstantInfo::NameAndType", name_and_type_index);
                    }
                } else {
                    todo!("使用默认的class_loader来load class")
                }
            } else {
                panic!("class: {:?}   static_field_index: {}  class_index: {} name_index: {} should point to ConstantInfo::ModifiedUTF8", class, static_field_index, class_index, name_index);
            }
        } else {
            panic!("class: {:?}   static_field_index: {}  class_index: {} should point to ConstantInfo::Class", class, static_field_index, class_index);
        }
    } else {
        panic!("class: {:?}   static_field_index: {} should point to ConstantInfo::MethodRef", class, static_field_index);
    }

    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}