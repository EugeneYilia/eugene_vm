use std::any::Any;
use std::any::TypeId;
use std::cell::RefMut;
use std::ops::Deref;
use std::rc::Rc;

use crate::core::class_loader::class_loader::ClassLoader;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::thread::Thread;

/// 实例方法调用参数收集形式
/// 参数从operand_stack收集的方式  第一个pop出来的是参数n 一直到参数1  之后再是对象引用object_ref

/// Invoke instance method; dispatch based on class
pub fn invoke_virtual(mut thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let class = Rc::clone(&stack_frame.class);
    let virtual_method_index = stack_frame.code_reader.read_u16() as usize;
    if let ConstantInfo::MethodRef { class_index, name_and_type_index } = class.constant_pool.get(virtual_method_index) {
        if let ConstantInfo::Class { name_index } = class.constant_pool.get(*class_index as usize) {
            if let ConstantInfo::ModifiedUTF8(ref class_name) = class.constant_pool.get(*name_index as usize) {
                let Class { class_loader, .. } = class.deref();
                if let Some(class_loader) = class_loader {
                    let class_ref = ClassLoader::load_class(Rc::clone(class_loader), class_name.to_owned(), &mut thread);
                    if let ConstantInfo::NameAndType { name_index, descriptor_index } = class.constant_pool.get(*name_and_type_index as usize) {
                        if let ConstantInfo::ModifiedUTF8(ref method_name) = class.constant_pool.get(*name_index as usize) {
                            if let ConstantInfo::ModifiedUTF8(ref method_descriptor) = class.constant_pool.get(*descriptor_index as usize) {
                                debug!("method_name: {}",method_name);
                                debug!("method_descriptor: {}",method_descriptor);

                                if let Some(method_ref) = class_ref.get_method(method_name, None, None) {
                                    error!("method: {:?}",method_ref);
                                    let args_types = &method_ref.args_type;
                                    let mut args: Option<Vec<VariableSlot>> = None;
                                    if let Some(arg_type_vec) = args_types {
                                        let variable_slot_vec: Vec<VariableSlot> = vec![];
                                        let args_size = arg_type_vec.len() + 1usize;
                                        let original_stack_size = stack_frame.operand_stack.get_length();
                                        let variable_slot_args: Vec<VariableSlot> = stack_frame.operand_stack.get_variable_slot_vec().drain((original_stack_size - args_size)..original_stack_size).collect();
                                        for (index, arg_type) in arg_type_vec.iter().enumerate() {
                                            let arg = variable_slot_args.get(index).unwrap();
                                            let arg_type = arg_type_vec.get(index).unwrap();
                                            error!("real arg: {:?}   typeId: {:?}",arg,arg.type_id());
                                            error!("required arg: {:?}   typeId: {:?}",arg_type,arg_type.type_id());
                                        }
                                    } else {
                                        args = None;
                                    }

                                    error!("method args type: {:?}",args_types);
                                    Thread::invoke_method(Rc::clone(&class_ref), method_ref, &mut thread, args);
                                } else {
                                    panic!("invoke_virtual error can't find method: {}", method_name);
                                }
                            } else {
                                panic!("virtual_method_index: {} name_and_type_index: {} descriptor_index: {} not point to ConstantInfo::ModifiedUTF8", virtual_method_index, name_and_type_index, descriptor_index);
                            }
                        } else {
                            panic!("virtual_method_index: {} name_and_type_index: {} name_index: {} not point to ConstantInfo::ModifiedUTF8", virtual_method_index, name_and_type_index, name_index);
                        }
                    } else {
                        panic!("virtual_method_index: {} name_and_type_index:{} not point to ConstantInfo::NameAndType", virtual_method_index, name_and_type_index);
                    }
                } else {
                    panic!("invoke_virtual class_loader is None");
                }
            } else {
                panic!("virtual_method_index: {} class_index: {} name_index: {} not point to ConstantInfo::ModifiedUTF8", virtual_method_index, class_index, name_index);
            }
        } else {
            panic!("virtual_method_index: {} class_index: {} not point to ConstantInfo::Class", virtual_method_index, class_index);
        }
    } else {
        panic!("virtual_method_index: {} not point to ConstantInfo::MethodRef", virtual_method_index);
    }

    // let value = stack_frame.operand_stack.pop_i32();
    // info!("{}", value);
    // println!("{}", value);
}

/// special handling for superclass, private, and instance initialization method invocations
pub fn invoke_special(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let class = Rc::clone(&stack_frame.class);
    let special_method_index = stack_frame.code_reader.read_u16() as usize;
    if let ConstantInfo::MethodRef { class_index, name_and_type_index } = class.constant_pool.get(special_method_index) {} else {
        panic!("special_method_index: {} not point to ConstantInfo::MethodRef", special_method_index);
    }
}

#[test]
fn test_drain() {
    let mut source = vec![1, 2, 3, 4, 5];
    println!("{:?}", source);
    // let result:Vec<i32> = source.drain(2..5).collect();
    let result: Vec<i32> = source.drain(2..5).collect();
    println!("{:?}", result);
    println!("{:?}", result.len());
    println!("{:?}", source);
    println!("{:?}", source.len());
}