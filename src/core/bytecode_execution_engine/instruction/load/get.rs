use std::cell::RefMut;
use std::ops::Deref;
use std::rc::Rc;

use crate::core::class_loader::class_loader::ClassLoader;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::thread::Thread;

pub fn get_static(mut thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    let static_field_index = stack_frame.code_reader.read_u16() as usize;
    debug!("static_field_index: {}", static_field_index);
    let class = Rc::clone(&stack_frame.class);
    if let ConstantInfo::FieldRef { class_index, name_and_type_index } = class.constant_pool.get(static_field_index) {
        debug!("class_index: {}", class_index);
        debug!("name_and_type_index: {}", name_and_type_index);
        if let ConstantInfo::Class { name_index } = class.constant_pool.get(*class_index as usize) {
            debug!("name_index: {}", name_index);
            if let ConstantInfo::ModifiedUTF8(ref class_name) = class.constant_pool.get(*name_index as usize) {
                debug!("class_name: {}", class_name);
                let Class { class_loader, .. } = class.deref();
                if let Some(class_loader) = class_loader {
                    let class_ref = ClassLoader::load_class(Rc::clone(class_loader), class_name.to_owned(), &mut thread);
                    if let ConstantInfo::NameAndType { name_index, descriptor_index } = class.constant_pool.get(*name_and_type_index as usize) {
                        if let ConstantInfo::ModifiedUTF8(ref field_name) = class.constant_pool.get(*name_index as usize) {
                            if let ConstantInfo::ModifiedUTF8(ref field_descriptor) = class.constant_pool.get(*descriptor_index as usize) {
                                debug!("static field field_name: {}  field_descriptor: {}", field_name, field_descriptor);
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
}