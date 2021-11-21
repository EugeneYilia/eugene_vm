use eugene_vm::bootstrap::bootstrap::start_jvm;
use eugene_vm::bootstrap::bootstrap_option::BootstrapOption;

fn main() {
    let class_name = "";
    let bootstrap_option = BootstrapOption::new(class_name, None, None, vec![]);

    start_jvm(bootstrap_option);
}
