use std::cell::{RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;

use crate::core::class_loader::class_loader::ClassLoader;
use crate::runtime::heap::object::Object;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::thread::Thread;

/// Memory for a new instance of that class is allocated from the garbage-collected heap
/// and the instance variables of the new object are initialized to their default initial values
/// The objectref, a reference to the instance, is pushed onto the operand stack.
///
/// The new instruction does not completely create a new instance;
/// instance creation is not completed until
/// an instance initialization method (§2.9) has been invoked on the uninitialized instance.
///               <init> method ↑
pub fn new(mut thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let class = Rc::clone(&stack_frame.class);

    let class_index = stack_frame.code_reader.read_u16();
    if let ConstantInfo::Class { name_index } = class.constant_pool.get(class_index as usize) {
        if let ConstantInfo::ModifiedUTF8(ref class_name) = class.constant_pool.get(*name_index as usize) {
            let Class { class_loader, .. } = class.deref();
            if let Some(class_loader) = class_loader {
                let class_ref = ClassLoader::load_class(Rc::clone(class_loader), class_name.to_owned(), &mut thread);
                let object = Rc::new(RefCell::new(Object::new(Rc::clone(&class_ref))));
                debug!("object fields: {:?}",object.deref().borrow().fields);
                debug!("object class_name: {:?}",object.deref().borrow().class.class_name);
                stack_frame.operand_stack.push(VariableSlot::ObjectReference(object));
            } else {
                panic!("instruction new error: classloader is None");
            }
        } else {
            panic!("instruction new error: {} not point to ConstantInfo::ModifiedUTF8", name_index);
        }
    } else {
        panic!("instruction new error: {} not point to ConstantInfo::Class", class_index);
    }
}