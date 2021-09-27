use EugeneVM::bootstrap::bootstrap_option::BootstrapOption;
use EugeneVM::bootstrap::bootstrap::start_jvm;

fn main() {
    let source_class_name = "";
    let class_name = source_class_name.replace(".","/");
    let bootstrap_option = BootstrapOption{
        class_name,
        user_classpath_option:None,
        boot_classpath_option:None,
        args:vec![]
    };
    start_jvm(bootstrap_option);
}
