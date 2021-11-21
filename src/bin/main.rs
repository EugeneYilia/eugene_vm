use EugeneVM::bootstrap::bootstrap_option::BootstrapOption;
use EugeneVM::bootstrap::bootstrap::start_jvm;

fn main() {
    let class_name = "";
    let bootstrap_option = BootstrapOption::new(class_name,None,None,vec![]);

    start_jvm(bootstrap_option);
}
