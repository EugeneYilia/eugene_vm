use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::bootstrap::bootstrap_option::BootstrapOption;
use crate::constants::access_flags::{ACCESS_PUBLIC, ACCESS_STATIC};
use crate::core::class_loader::class_loader::ClassLoader;
use crate::core::classpath::classpath::ClassPath;
use crate::runtime::thread::Thread;

// class_name 主函数入口
// user_classpath and boot_classpath需要先解析出来
pub fn start_jvm(bootstrap_option: BootstrapOption) {
    init_jvm();

    let main_thread = Rc::new(RefCell::new(Thread::new(None)));

    let classpath = ClassPath::parse_classpath(bootstrap_option.boot_classpath_option, bootstrap_option.user_classpath_option);
    let class_loader: Rc<RefCell<ClassLoader>> = Rc::new(RefCell::new(ClassLoader::new(classpath, Rc::clone(&main_thread))));
    let class_ref = ClassLoader::load_class(class_loader, bootstrap_option.class_name, &mut main_thread.deref().borrow_mut());

    if let Some(method_ref) = class_ref.get_method("main", Some("([Ljava/lang/String;)V"), Some(vec![ACCESS_PUBLIC, ACCESS_STATIC])) {
        Thread::start_thread(class_ref, method_ref, Rc::clone(&main_thread))
    } else {
        panic!("{:?} can not find main method", class_ref);
    }
}

fn init_jvm() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
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