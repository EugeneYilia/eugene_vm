use std::cell::RefMut;
use std::ops::Deref;
use std::rc::Rc;

use crate::core::class_loader::class_loader::ClassLoader;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
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

                                // let method_ref = class_ref.get_method();
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

    let value = stack_frame.operand_stack.pop_i32();
    info!("{}", value);
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