use std::cell::RefCell;
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
    let classpath = ClassPath::parse_classpath(bootstrap_option.boot_classpath_option, bootstrap_option.user_classpath_option);
    let class_loader: Rc<RefCell<ClassLoader>> = Rc::new(RefCell::new(ClassLoader::new(classpath)));
    let class_ref = ClassLoader::load_class(class_loader, bootstrap_option.class_name);

    let method_ref = class_ref.get_method("main", "([Ljava/lang/String;)V", vec![ACCESS_PUBLIC, ACCESS_STATIC]);
    start_interpret(class_ref, method_ref)
}

fn start_interpret(class: Rc<Class>, method: Rc<Method>) {
    let mut main_thread = Thread::new(None);
    let init_stack_frame = StackFrame::new(class, method);
    main_thread.push_stack_frame(init_stack_frame);
    execute_thread(main_thread);
}

fn execute_thread(mut thread: Thread) {
    let mut pc = 0usize;
    while thread.has_stack_frame() {
        let instruction_execute_result = engine::execute_instruction(&mut thread, pc);
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