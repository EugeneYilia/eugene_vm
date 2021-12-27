use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;

use crate::bootstrap::bootstrap_option::BootstrapOption;
use crate::constants::access_flags::{ACCESS_PUBLIC, ACCESS_STATIC};
use crate::core::bytecode_execution_engine::engine;
use crate::core::class_loader::class_loader::ClassLoader;
use crate::core::classpath::classpath::ClassPath;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::class::method::Method;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

// class_name 主函数入口
// user_classpath and boot_classpath需要先解析出来
pub fn start_jvm(bootstrap_option: BootstrapOption) {
    let main_thread = Rc::new(RefCell::new(Thread::new(None)));

    let classpath = ClassPath::parse_classpath(bootstrap_option.boot_classpath_option, bootstrap_option.user_classpath_option);
    let class_loader: Rc<RefCell<ClassLoader>> = Rc::new(RefCell::new(ClassLoader::new(classpath, Rc::clone(&main_thread))));
    let class_ref = ClassLoader::load_class(class_loader, bootstrap_option.class_name);

    let method_ref = class_ref.get_method("main", "([Ljava/lang/String;)V", vec![ACCESS_PUBLIC, ACCESS_STATIC]);
    invoke_method(class_ref, method_ref, Rc::clone(&main_thread))
}

pub fn invoke_method(class: Rc<Class>, method: Rc<Method>, thread: Rc<RefCell<Thread>>) {
    let stack_frame = StackFrame::new(class, method);
    thread.deref().borrow_mut().push_stack_frame(stack_frame);
    execute_thread(Rc::clone(&thread));
}

fn execute_thread(thread: Rc<RefCell<Thread>>) {
    let mut pc = 0usize;
    while thread.deref().borrow().has_stack_frame() {
        let instruction_execute_result = engine::execute_instruction(thread.deref().borrow_mut(), pc);
        pc = instruction_execute_result.new_pc;
    }
}

#[cfg(test)]
mod tests {
    use crate::bootstrap::bootstrap::start_jvm;
    use crate::bootstrap::bootstrap_option::BootstrapOption;

    #[test]
    fn test_invoke_virtual() {
        let class_name = "TestAdd";
        let user_classpath = Some("eugene_test/src_code/mine".to_owned());
        let boot_classpath = Some("eugene_test/src_code/eugene_rt".to_owned());
        let bootstrap_option = BootstrapOption::new(class_name, user_classpath, boot_classpath, vec![]);

        start_jvm(bootstrap_option);
    }
}